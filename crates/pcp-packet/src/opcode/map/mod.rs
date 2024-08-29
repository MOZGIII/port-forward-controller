//! `MAP` opcode data.

mod request;
mod response;

pub use request::Data as Request;
pub use response::Data as Response;

/// The length in bytes.
pub const LEN: usize = crate::ROW_SIZE * 9;

/// The buffer of the size to fit the data.
pub type Buffer = [u8; LEN];
