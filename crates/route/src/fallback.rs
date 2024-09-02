/// Find default gateway for a given interface.
///
/// Mock for non-linux systems.
pub async fn gateway_for(
    _interface: std::net::IpAddr,
) -> Result<Option<std::net::IpAddr>, std::io::Error> {
    Ok(None)
}

/// A mock error.
pub struct GatewayForError;
