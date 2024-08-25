//! Core abstractions for the port control client.
//!
//! Modelled after the RFC 6687 - Port Control Protocol.
//!
//! <https://en.wikipedia.org/wiki/Port_Control_Protocol>
//! <https://datatracker.ietf.org/doc/html/rfc6887>

pub mod map;
pub mod pcp;

/// The resource lifetime in seconds.
///
/// `0` mean the resource will be deleted immediately.
pub type LifetimeSeconds = u32;

/// The protocol number.
///
/// Protocol numbers list in maintained by IANA.
/// See <https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml>.
pub type Protocol = u8;

/// The port number.
pub type Port = u16;

/// Protocol consts.
pub mod protocol {
    #![allow(missing_docs)]

    use super::Protocol;

    pub const ANY: Protocol = 0;
    pub const TCP: Protocol = 6;
    pub const UDP: Protocol = 17;
    pub const SCTP: Protocol = 132;
    pub const DCCP: Protocol = 33;
}

/// Port consts.
pub mod port {
    use super::Port;

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

/// An indication of the lifetime of an error.
pub trait ErrorLifetime {
    /// Returns the amount of seconds within which this error will likely repeat if we retry
    /// the request
    fn lifetime(&self) -> Option<LifetimeSeconds>;
}
