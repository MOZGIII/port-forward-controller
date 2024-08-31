//! Tooling to convert CRD to PCP client types.

use std::sync::Arc;

use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// The error that can occur while converting the CRD into a PCP type.
#[derive(Debug, thiserror::Error)]
pub enum ConversionError {
    /// The protocol is a number that is too big or negative.
    #[error("invalid protocol number: {0}")]
    InvalidProtocolNumber(std::num::TryFromIntError),

    /// The protocol is a string that we don't recognize.
    #[error("unknown protocol name: {0}")]
    UnknownProtocolName(Arc<str>),
}

/// Convert the CRD into a PCP type.
#[derive(Debug, Clone)]
pub struct Converter {
    /// The nonce to use for all the managed mappings.
    pub nonce: pcp_primitives::Nonce,

    /// The lifetime to request for the mappings.
    pub lifetime: pcp_primitives::LifetimeSeconds,
}

impl Converter {
    /// Create a PCP client mapping from a corresponding CRD.
    pub fn mapping_from_crd(
        &self,
        crd: &crd::PCPMap,
    ) -> Result<pcp_client::Mapping, ConversionError> {
        let id = self.mapping_id_from_crd(crd)?;
        let params = self.mapping_params_from_crd(crd)?;

        Ok(pcp_client::Mapping { id, params })
    }

    /// Create a PCP client mapping ID from a corresponding CRD.
    pub fn mapping_id_from_crd(
        &self,
        crd: &crd::PCPMap,
    ) -> Result<pcp_client::mapping::Id, ConversionError> {
        let crd::PCPMapSpec {
            protocol,
            from: _,
            to,
        } = &crd.spec;

        let protocol = match protocol {
            IntOrString::Int(val) => pcp_primitives::Protocol::try_from(*val)
                .map_err(ConversionError::InvalidProtocolNumber)?,
            IntOrString::String(val) => match val.to_ascii_lowercase().as_str() {
                "any" => pcp_consts::protocol::ANY,
                "tcp" => pcp_consts::protocol::TCP,
                "udp" => pcp_consts::protocol::UDP,
                "sctp" => pcp_consts::protocol::SCTP,
                "dccp" => pcp_consts::protocol::DCCP,
                _ => return Err(ConversionError::UnknownProtocolName(val.to_owned().into())),
            },
        };

        Ok(pcp_client::mapping::Id {
            protocol,
            internal_ip: pcp_ip_conv::unify(to.ip()),
            internal_port: to.port(),
            nonce: self.nonce,
        })
    }

    /// Create PCP client mapping params from a corresponding CRD.
    pub fn mapping_params_from_crd(
        &self,
        crd: &crd::PCPMap,
    ) -> Result<pcp_client::mapping::Params, ConversionError> {
        let crd::PCPMapSpec {
            protocol: _,
            from,
            to: _,
        } = &crd.spec;

        Ok(pcp_client::mapping::Params {
            lifetime: self.lifetime,
            external_port: *from,
            exteranl_ip: pcp_primitives::Address::UNSPECIFIED, // TODO: use the value from status if present
            third_party: None,
            prefer_failure: Some(pcp_client::mapping::option::PcpOption {
                is_optional: false,
                payload: (),
            }),
            filters: None,
        })
    }
}
