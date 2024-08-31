//! Core abstractions for the port control client.
//!
//! Modelled after the RFC 6687 - Port Control Protocol.
//!
//! <https://en.wikipedia.org/wiki/Port_Control_Protocol>
//! <https://datatracker.ietf.org/doc/html/rfc6887>

use core::future::Future;

/// The size of any PCP packet.
pub const PCP_PACKET_SIZE: usize = 1100;

/// Information about a received packet.
#[derive(Debug)]
pub struct RecvInfo {
    /// The address of the remote socket endpoint from where the packet was sent to us.
    pub src: std::net::SocketAddr,

    /// The address of the local socket endpoint at which we have received the packet.
    pub dst: std::net::SocketAddr,
}

/// The PCP client transport.
pub trait Transport {
    /// Send a PCP request to the server.
    fn send<'a>(
        &'a self,
        to: std::net::SocketAddr,
        request: &'a [u8; PCP_PACKET_SIZE],
    ) -> impl Future<Output = Result<(), std::io::Error>> + Send + 'a;

    /// Receive a PCP response from the server.
    fn recv<'a>(
        &'a self,
        response: &'a mut [u8; PCP_PACKET_SIZE],
    ) -> impl Future<Output = Result<RecvInfo, std::io::Error>> + Send + 'a;
}

/// The PCP client runtime.
pub trait Runtime {
    /// The future for sleeping.
    type SleepFuture: std::future::Future<Output = ()>;

    /// Sleep for a given amount of time.
    ///
    /// Does not bound the runtime with duration of the sleep.
    /// If this is needed, an internal representation that enabled it is to be used,
    /// `Arc<InnerRuntime>`.
    fn sleep(&self, duration: std::time::Duration) -> Self::SleepFuture;

    /// Spawn the given future into a background task.
    ///
    /// Does not bound the runtime with the execution of the future, instead requires
    /// the future to be `'static`.
    fn spawn_background(&self, fut: impl Future<Output = ()> + Send + 'static);
}
