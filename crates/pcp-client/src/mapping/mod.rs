pub mod option;

use pcp_primitives::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id {
    /// Protocol.
    ///
    /// Specified by the client, server must acceept or reject with `UNSUPP_PROTOCOL`.
    pub protocol: Protocol,

    /// Internal IP.
    ///
    /// Specified by the client, but can't be chosen freely as the server will check the packets
    /// it is receiving are sent from this `src`, and will also send the reply to that packet.
    pub internal_ip: Address,

    /// Internal port.
    ///
    /// Specified by the client, server must accept or reject with `UNSUPP_PROTOCOL`.
    pub internal_port: Port,

    /// The mapping nonce.
    ///
    /// A unique random sequence of bytes.
    pub nonce: Nonce,
}

#[derive(Debug)]
pub struct Params {
    /// Lifetime (in seconds).
    ///
    /// Specified by the client.
    pub lifetime: LifetimeSeconds,

    /// External port.
    ///
    /// Specified by the server, but the client can suggest a port it wants.
    pub external_port: Port,

    /// External IP.
    ///
    /// Technically specified by the server, but the client can suggest the IP address it wants.
    pub exteranl_ip: Address,

    /// Third Party option.
    ///
    /// Specified by the client. Server must accept or ``
    ///
    /// Indicates this mapping does not have the Client IP as its Internal IP, but the server
    /// should respect that and create the port forward.
    ///
    /// This MUST NOT be implemented without a separate authorization.
    pub third_party: Option<option::ThirdParty>,

    /// Prefer Failure option.
    ///
    /// ...
    pub prefer_failure: Option<option::PreferFailure>,

    /// ...
    pub filters: Option<option::Filters>,
}

#[derive(Debug)]
pub struct Mapping {
    /// The fields that constitute a mapping ID.
    pub id: Id,

    /// The fields that are only used for selecting mapping params.
    pub params: Params,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Incoming {
    pub received_on: Address,
    pub packet_header: pcp_codec::data::response::Header,
    pub packet_opcode: pcp_codec::data::response::Map,
}

impl Incoming {
    pub fn id(&self) -> Id {
        Id {
            protocol: self.packet_opcode.protocol,
            internal_ip: self.received_on,
            internal_port: self.packet_opcode.internal_port,
            nonce: self.packet_opcode.mapping_nonce,
        }
    }
}

impl pcp_lifecycle::Mapping for Mapping {
    fn is_same_mapping_instance(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl pcp_lifecycle::Incoming<Mapping> for Incoming {
    fn is_same_exposed_resource(&self, other: &Mapping) -> bool {
        self.received_on == other.id.internal_ip
            && self.packet_opcode.protocol == other.id.protocol
            && self.packet_opcode.internal_port == other.id.internal_port
    }
}

impl pcp_lifecycle::cleanup::Into for Mapping {
    type Mapping = Self;

    fn into_cleanup(mut self) -> Self::Mapping {
        self.params.lifetime = 0;
        self
    }
}

impl pcp_lifecycle::cleanup::Maybe for Incoming {
    fn is_cleanup(&self) -> bool {
        self.packet_header.result_code == pcp_consts::result_code::SUCCESS
            && self.packet_header.lifetime == 0
    }
}
