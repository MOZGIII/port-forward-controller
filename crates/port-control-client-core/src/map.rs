//! The port map operation.

use std::net::IpAddr;

use crate::*;

/// The port map operation.
pub trait Client {
    /// The error that can occur at the [`Self::map`] fn.
    type Error: std::error::Error + ErrorLifetime;

    /// Send a port map request to the server and provide the response.
    fn map(
        &self,
        req: Request,
    ) -> impl std::future::Future<Output = Result<Response, Self::Error>> + Send;
}

/// A request to the port map server.
#[derive(Debug, Clone)]
pub struct Request {
    /// The protocol to map the ports at.
    pub protocol: Protocol,

    /// The port to direct the mapped traffic to on the node.
    ///
    /// The IP of this endpoint will effectively be the address that the port mapping server sees
    /// this request arriving from.
    pub node_port: Port,

    /// The port to open on the gateway.
    ///
    /// The IP of this endpoint will typically be the external address of the gateway running
    /// the port mapping server.
    ///
    /// Set this to the port that you want to see from the outside.
    pub gateway_port: Port,

    /// The lifetime of the mapping.
    ///
    /// Pass `0` to clear the mapping.
    ///
    /// To maintain a consistent port mapping without interruption, re-request the mapping before
    /// the lifetime expires.
    pub lifetime: LifetimeSeconds,
}

/// A response from the port map server.
#[derive(Debug, Clone)]
pub struct Response {
    /// The effective protocol of the mapping.
    pub protocol: Protocol,

    /// The node port that the mapping is created for.
    pub node_port: Port,

    /// The effective gateway port that was assigned for the mapping.
    pub gateway_port: Port,

    /// The effective external IP that the mapping is available on.
    pub gateway_ip: IpAddr,

    /// The effective lifetime the server created the mapping with.
    pub lifetime: LifetimeSeconds,
}
