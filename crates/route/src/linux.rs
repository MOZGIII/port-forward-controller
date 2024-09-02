/// An error that can occur when getting the gateway for a given interface.
#[derive(Debug, thiserror::Error)]
pub enum GatewayForError {
    /// Opening netlink socket.
    #[error("opening netlink socket: {0}")]
    NetlinkOpen(std::io::Error),

    /// Reading a route.
    #[error("reading a route: {0}")]
    ReadingRoute(rtnetlink::Error),
}

/// Find default gateway for a given interface.
pub async fn gateway_for(
    interface: std::net::IpAddr,
) -> Result<Option<std::net::IpAddr>, GatewayForError> {
    use futures::TryStreamExt as _;

    let (connection, handle, _) =
        rtnetlink::new_connection().map_err(GatewayForError::NetlinkOpen)?;
    let _connection_guard = Guard(tokio::spawn(connection));

    let ip_version = match &interface {
        std::net::IpAddr::V4(_) => rtnetlink::IpVersion::V4,
        std::net::IpAddr::V6(_) => rtnetlink::IpVersion::V6,
    };

    let mut request = handle.route().get(ip_version);

    request.message_mut().header.destination_prefix_length = 0;
    request.message_mut().header.kind = netlink_packet_route::route::RouteType::Unspec;

    let mut routes = request.execute();

    loop {
        let maybe_route = routes
            .try_next()
            .await
            .map_err(GatewayForError::ReadingRoute)?;
        let Some(route) = maybe_route else { break };

        tracing::debug!(message = "got route", ?route);

        let Some(gateway) = try_route(interface, route) else {
            continue;
        };

        tracing::info!(message = "gateway selected", ?gateway);

        return Ok(Some(gateway));
    }

    Ok(None)
}

/// Filter the route and extract a gateway if it fits.
fn try_route(
    interface: std::net::IpAddr,
    route: netlink_packet_route::route::RouteMessage,
) -> Option<std::net::IpAddr> {
    let is_default_route = route.header.destination_prefix_length == 0;
    if !is_default_route {
        return None;
    }

    let gateway = route
        .attributes
        .iter()
        .find_map(|attribute| match attribute {
            netlink_packet_route::route::RouteAttribute::Gateway(val) => Some(val),
            _ => None,
        })?;

    let maybe_source = route
        .attributes
        .iter()
        .find_map(|attribute| match attribute {
            netlink_packet_route::route::RouteAttribute::PrefSource(val) => Some(val),
            _ => None,
        });

    let source_prefix = route.header.source_prefix_length;
    let source_matched = match (maybe_source, interface) {
        (
            Some(netlink_packet_route::route::RouteAddress::Inet(source)),
            std::net::IpAddr::V4(interface),
        ) => ipv4_matches(*source, source_prefix, interface),

        (
            Some(netlink_packet_route::route::RouteAddress::Inet6(source)),
            std::net::IpAddr::V6(interface),
        ) => ipv6_matches(*source, source_prefix, interface),

        // Allow if the interface has no source specified.
        (None, _) => true,

        _ => false,
    };
    if !source_matched {
        return None;
    }

    let gateway = match gateway {
        netlink_packet_route::route::RouteAddress::Inet(ip) => std::net::IpAddr::V4(*ip),
        netlink_packet_route::route::RouteAddress::Inet6(ip) => std::net::IpAddr::V6(*ip),
        _ => return None,
    };

    Some(gateway)
}

/// Check if candidate is covered by the mask with a given prefix.
fn ipv4_matches(mask: std::net::Ipv4Addr, prefix: u8, candidate: std::net::Ipv4Addr) -> bool {
    // TODO
    tracing::debug!(message = "matching ipv4", ?mask, %prefix, ?candidate);
    mask == candidate
}

/// Check if candidate is covered by the mask with a given prefix.
fn ipv6_matches(mask: std::net::Ipv6Addr, prefix: u8, candidate: std::net::Ipv6Addr) -> bool {
    // TODO
    tracing::debug!(message = "matching ipv6", ?mask, %prefix, ?candidate);
    mask == candidate
}

/// Automatic cleanup for the spawned netlink connection handler.
struct Guard(tokio::task::JoinHandle<()>);

impl Drop for Guard {
    fn drop(&mut self) {
        self.0.abort()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filtering() {
        use netlink_packet_route::{route::*, AddressFamily};

        let header = RouteHeader {
            address_family: AddressFamily::Inet,
            destination_prefix_length: 0,
            source_prefix_length: 0,
            tos: 0,
            table: 254,
            protocol: RouteProtocol::Dhcp,
            scope: RouteScope::Universe,
            kind: RouteType::Unicast,
            flags: vec![],
        };

        let mut sample = RouteMessage::default();
        sample.header = header;
        sample.attributes = vec![
            RouteAttribute::Table(254),
            RouteAttribute::Priority(1024),
            RouteAttribute::PrefSource(RouteAddress::Inet(std::net::Ipv4Addr::new(
                192, 168, 0, 10,
            ))),
            RouteAttribute::Gateway(RouteAddress::Inet(std::net::Ipv4Addr::new(192, 168, 0, 1))),
            RouteAttribute::Oif(28),
        ];

        assert_eq!(
            try_route(std::net::Ipv4Addr::new(192, 168, 0, 10).into(), sample),
            Some(std::net::Ipv4Addr::new(192, 168, 0, 1).into())
        );
    }
}
