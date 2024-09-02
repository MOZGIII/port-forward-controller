//! System routing table utils.

/// Linux implementation using netlink.
#[cfg(target_os = "linux")]
mod linux;

/// Fallback implementation.
#[cfg(not(target_os = "linux"))]
mod fallback;

#[cfg(target_os = "linux")]
use self::linux as platform;

#[cfg(not(target_os = "linux"))]
use self::fallback as platform;

pub use self::platform::{gateway_for, GatewayForError};
