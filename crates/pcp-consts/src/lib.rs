//! Core abstractions for the port control client.
//!
//! Modelled after the RFC 6687 - Port Control Protocol.
//!
//! <https://en.wikipedia.org/wiki/Port_Control_Protocol>
//! <https://datatracker.ietf.org/doc/html/rfc6887>

pub use pcp_primitives as primitives;

pub mod protocol;
pub mod result_code;

/// Port consts.
pub mod port {
    use super::primitives::Port;

    /// Any port.
    ///
    /// Usage:
    ///
    /// - when used in the port mapping request as the node port -
    ///   indicates that the port mapping has to apply for all protocols;
    /// - when used in the port mapping request as the gateway port -
    ///   indicates that any available port can be chosen by the NAT server.
    ///
    /// This is designed to match the PCP semantics, so
    /// see <https://tools.ietf.org/html/rfc6887#section-11.1>.
    pub const ANY: Port = 0;
}
