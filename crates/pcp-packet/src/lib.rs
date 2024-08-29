//! PCP protocol implementation.

#![allow(missing_docs, clippy::missing_docs_in_private_items)]
#![no_std]

pub mod header;
mod meta;
pub mod opcode;
pub mod option;
mod r_and_opcode;

pub const LEN: usize = 1100;

pub type Buffer = [u8; LEN];

pub use meta::Meta;
pub use r_and_opcode::RAndOpcode;

const ROW_SIZE: usize = 4;
