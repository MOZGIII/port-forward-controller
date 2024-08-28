//! Protocol consts.

#![allow(missing_docs)]

use super::primitives::Protocol;

pub const ANY: Protocol = 0;
pub const TCP: Protocol = 6;
pub const UDP: Protocol = 17;
pub const SCTP: Protocol = 132;
pub const DCCP: Protocol = 33;
