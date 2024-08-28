//! Core abstractions for the port control client.
//!
//! Modelled after the RFC 6687 - Port Control Protocol.
//!
//! <https://en.wikipedia.org/wiki/Port_Control_Protocol>
//! <https://datatracker.ietf.org/doc/html/rfc6887>

#![no_std]

pub use pcp_primitives as primitives;

/// The version used by the PCP protocol.
///
/// <https://datatracker.ietf.org/doc/html/rfc6887#section-9>
pub const VERSION: primitives::PcpVersion = 2;

/// The standard port used by the server of the PCP protocol itself.
pub const PCP_SERVER_PORT: u16 = 5351;

/// The standard port used by the client of the PCP protocol itself for listening for
/// the `ANNOUNCE` packets.
pub const PCP_CLIENT_LISTEN_PORT: u16 = 5350;

pub mod result_code;

/// Protocol consts.
pub mod protocol {
    #![allow(missing_docs)]

    use super::primitives::Protocol;

    pub const ANY: Protocol = 0;
    pub const TCP: Protocol = 6;
    pub const UDP: Protocol = 17;
    pub const SCTP: Protocol = 132;
    pub const DCCP: Protocol = 33;
}

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

/// Opcode consts.
pub mod opcode {
    #![allow(missing_docs)]

    use super::primitives::Opcode;

    pub const ANNOUNCE: Opcode = 0;
    pub const MAP: Opcode = 1;
    pub const PEER: Opcode = 2;
}
