//! Core abstractions for the port control client.
//!
//! Modelled after the RFC 6687 - Port Control Protocol.
//!
//! <https://en.wikipedia.org/wiki/Port_Control_Protocol>
//! <https://datatracker.ietf.org/doc/html/rfc6887>

use std::{
    borrow::{Borrow, BorrowMut},
    future::Future,
};

/// The size of any PCP packet.
const PCP_PACKET_SIZE: usize = 1100;

/// The PCP client.
pub trait Client {
    /// Send a PCP request to the server.
    fn send<'req, Request>(
        &self,
        req: Request,
    ) -> impl Future<Output = Result<(), std::io::Error>> + Send + 'req
    where
        Request: Borrow<[u8; PCP_PACKET_SIZE]> + 'req;

    /// Receive a PCP response from the server.
    fn recv<'res, Response>(
        &self,
        response: Response,
    ) -> impl Future<Output = Result<(), std::io::Error>> + Send + 'res
    where
        Response: BorrowMut<[u8; PCP_PACKET_SIZE]> + 'res;
}
