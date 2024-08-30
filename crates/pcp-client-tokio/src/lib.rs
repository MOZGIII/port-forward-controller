//! The PCP client building blocks using `tokio` networking and timers.

use std::net::SocketAddr;

use pcp_client_core::PCP_PACKET_SIZE;

/// The client transport.
#[derive(Debug)]
pub struct Transport {
    /// The UDP socket to use.
    pub socket: tokio::net::UdpSocket,

    /// The local address to report as `dst` for incoming packets.
    ///
    /// Note this is wrong, and this should be determined dynamically via `IP_PKTINFO`, but
    /// the correct implementation is left for later.
    pub local_address: std::net::SocketAddr,
}

impl pcp_client_core::Transport for Transport {
    async fn send<'a>(
        &'a self,
        to: SocketAddr,
        request: &'a [u8; PCP_PACKET_SIZE],
    ) -> Result<(), std::io::Error> {
        tracing::debug!(message = "sending packet", ?to, ?request);

        let len = self.socket.send_to(request, to).await?;

        if len != PCP_PACKET_SIZE {
            return Err(std::io::Error::other("unable to write full packet"));
        }

        Ok(())
    }

    async fn recv<'a>(
        &'a self,
        response: &'a mut [u8; PCP_PACKET_SIZE],
    ) -> Result<pcp_client_core::RecvInfo, std::io::Error> {
        let (len, from) = self.socket.recv_from(response).await?;

        tracing::debug!(message = "received packet", ?from, ?response);

        if len != PCP_PACKET_SIZE {
            return Err(std::io::Error::other("invalid packet received"));
        }

        Ok(pcp_client_core::RecvInfo {
            src: from,
            dst: self.local_address, // FIXME: use `IP_PKTINFO` to properly detect this
        })
    }
}

/// The `tokio`-powered client runtime.
#[derive(Debug, Clone, Copy)]
pub struct Runtime;

impl pcp_client_core::Runtime for Runtime {
    type SleepFuture = tokio::time::Sleep;

    fn sleep(&self, duration: std::time::Duration) -> Self::SleepFuture {
        tokio::time::sleep(duration)
    }

    fn spawn_background(&self, fut: impl std::future::Future<Output = ()> + Send + 'static) {
        tokio::spawn(fut);
    }
}
