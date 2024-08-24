//! CRDs

use std::net::SocketAddr;

use garde::Validate;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A definition of the [`PCPMap`] custom resource.
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, Validate, JsonSchema)]
#[kube(
    group = "port-forward.io",
    version = "v1alpha1",
    kind = "PCPMap",
    namespaced
)]
#[kube(status = "PCPMapStatus")]
#[kube(printcolumn = r#"{"name":"Port", "jsonPath": ".spec.from", "type": "int"}"#)]
pub struct PCPMapSpec {
    /// The protocol to forward.
    #[garde(skip)] // TODO: #[garde(dive)]
    pub protocol: Protocol,

    /// The port number to forward from.
    #[garde(skip)] // TODO: #[garde(dive)]
    pub from: PortNumber,

    /// The address to forward to.
    #[garde(skip)] // TODO: #[garde(dive)]
    pub to: SocketAddr,
}

/// A definition of the status for the [`PCPMap`] custom resource.
#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
pub struct PCPMapStatus {
    /// Is port forward currently active.
    pub active: bool,

    /// The effective protocol number.
    pub protocol_number: ProtocolNumber,

    /// The endpoint to reach the forwarded port from the outside.
    pub external_endpoint: Option<SocketAddr>,
}

/// A port number.
pub type PortNumber = u16;

/// IANA protocol number.
///
/// See <https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml>.
pub type ProtocolNumber = u8;

/// The protocol number or name.
pub type Protocol = k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
