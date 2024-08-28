#![allow(missing_docs, clippy::missing_docs_in_private_items)]

pub type Protocol = u8;
pub type Port = u16;
pub type LifetimeSeconds = u32;
pub type Address = core::net::Ipv6Addr;
pub type Nonce = [u8; 12];
pub type PrefixLength = u8;
pub type ResultCode = u8;
