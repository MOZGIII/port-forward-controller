#![allow(missing_docs, clippy::missing_docs_in_private_items)]

pub enum PcpServerAddress {
    Explicit(std::net::SocketAddr),
    PortOnly(u16),
}

impl PcpServerAddress {
    pub fn from_env() -> Result<Self, color_eyre::Report> {
        if let Some(socket_address) = envfury::maybe("PCP_SERVER_ADDR")? {
            return Ok(Self::Explicit(socket_address));
        }

        let port = envfury::or("PCP_SERVER_PORT", pcp_consts::PCP_SERVER_PORT)?;

        match envfury::maybe("PCP_SERVER_IP_ADDR")? {
            Some(ip_address) => Ok(Self::Explicit(std::net::SocketAddr::new(ip_address, port))),
            None => Ok(Self::PortOnly(port)),
        }
    }
}
