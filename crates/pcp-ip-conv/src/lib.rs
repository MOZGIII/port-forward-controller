//! Utilities for the PCP-protocol related IP address conversions.

#![no_std]

/// Represent IPv4 addresses as IPv6 mapped addresses.
#[inline]
pub fn unify(ip: core::net::IpAddr) -> core::net::Ipv6Addr {
    match ip {
        core::net::IpAddr::V4(ip) => ip.to_ipv6_mapped(),
        core::net::IpAddr::V6(ip) => ip,
    }
}

/// Convert the IPv6 mapped address back into IPv4.
#[inline]
pub fn split(ip: core::net::Ipv6Addr) -> core::net::IpAddr {
    ip.to_canonical()
}
