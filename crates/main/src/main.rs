//! Main entrypoint.

mod env;

use std::sync::Arc;

use color_eyre::eyre::OptionExt;
use env::PcpServerAddress;

#[tokio::main]
async fn main() -> Result<(), color_eyre::Report> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let bind_ip_address: std::net::IpAddr =
        envfury::or("BIND_ADDR", std::net::Ipv6Addr::UNSPECIFIED.into())?;
    let bind_port: u16 = envfury::or("BIND_PORT", pcp_consts::PCP_CLIENT_LISTEN_PORT)?;
    let bind_socket_address = std::net::SocketAddr::new(bind_ip_address, bind_port);

    let local_ip_address: std::net::IpAddr = envfury::must("LOCAL_ADDR")?;

    let pcp_server_address = PcpServerAddress::from_env()?;

    let keepalive_interval_secs = envfury::or("KEEPALIVE_INTERVAL_SECS", 30)?;
    let keepalive_interval = std::time::Duration::from_secs(keepalive_interval_secs);

    // ---

    let pcp_server_address = match pcp_server_address {
        env::PcpServerAddress::Explicit(socket_address) => socket_address,
        env::PcpServerAddress::PortOnly(port) => {
            let maybe_ip_address = route::gateway_for(local_ip_address).await?;
            let ip_address = maybe_ip_address
                .ok_or_eyre("unable to detect PCP server IP address, specify it manually")?;
            std::net::SocketAddr::new(ip_address, port)
        }
    };

    let pcp_client_socket = tokio::net::UdpSocket::bind(bind_socket_address).await?;

    let local_socket_address = {
        let effective_socket_addr = pcp_client_socket.local_addr()?;
        std::net::SocketAddr::new(local_ip_address, effective_socket_addr.port())
    };
    tracing::info!(message = "PCP client local address", %local_socket_address);

    let kube_client = kube::Client::try_default().await?;

    let (notifications_tx, notifications_rx) = tokio::sync::mpsc::channel(0xff);

    let pcp_client_transport = pcp_client_tokio::Transport {
        socket: pcp_client_socket,
        local_address: local_socket_address,
    };
    let pcp_client = pcp_client::Client {
        runtime: pcp_client_tokio::Runtime,
        transport: pcp_client_transport,
        server_address: pcp_server_address,
        mappings: Default::default(),
        keepalive_interval,
        notifications_tx,
    };

    let (command_tx, command_rx) = tokio::sync::mpsc::channel(1);

    let mapping_lifetime = keepalive_interval
        .as_secs()
        .try_into()
        .unwrap_or(u32::MAX)
        .saturating_mul(2);

    let converter = crd_controller::pcp::Converter {
        nonce: [0; 12],
        lifetime: mapping_lifetime,
    };

    let reconciler_ctx = crd_controller::reconciler::Context {
        params: crd_controller::reconciler::Params::default(),
        command_tx,
        k8s_client: kube_client.clone(),
        converter: converter.clone(),
    };
    let reconciler_ctx = Arc::new(reconciler_ctx);

    let all_crds_api = kube::Api::all(kube_client.clone());

    let controller = kube::runtime::Controller::new(
        all_crds_api.clone(),
        kube::runtime::watcher::Config::default(),
    );

    let indexer = crd_controller::status::indexer::new(converter);

    let status_listener = crd_controller::status::Listener {
        indexer,
        kube_client,
    };

    use kube::runtime::WatchStreamExt;
    let all_crds_watch =
        kube::runtime::watcher(all_crds_api, kube::runtime::watcher::Config::default())
            .default_backoff();

    // ---

    tokio::spawn(crd_controller::run(controller, reconciler_ctx));

    tokio::spawn(status_listener.lifecycle_loop(all_crds_watch, notifications_rx));

    tracing::info!(message = "startup complete");

    // FIXME: this one should actually be running in the spawn as well.
    // See <https://github.com/rust-lang/rust/issues/96865>.
    pcp_client.into_lifecycle_loop(command_rx).await;

    Ok(())
}
