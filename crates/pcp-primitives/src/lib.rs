#![allow(missing_docs, clippy::missing_docs_in_private_items)]
#![no_std]

pub type Protocol = u8;
pub type Port = u16;
pub type LifetimeSeconds = u32;
pub type Address = core::net::Ipv6Addr;
pub type Nonce = [u8; 12];
pub type PrefixLength = u8;
pub type ResultCode = u8;
pub type PcpVersion = u8;
pub type EpochTime = u32;
pub type OptionCode = u8;

/// A 7-bit value specifying the operation.
pub type Opcode = u8;
