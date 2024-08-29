pub mod check;

use const_sub_array::SubArray;

use crate::data;

pub struct State<'p> {
    packet: &'p pcp_packet::Buffer,
}

impl<'p> State<'p> {
    pub fn new(packet: &'p pcp_packet::Buffer) -> Self {
        Self { packet }
    }

    pub fn meta(&self) -> &'p pcp_packet::Meta {
        let output = self
            .packet
            .sub_array_ref::<0, { size_of::<pcp_packet::Meta>() }>();
        bytemuck::must_cast_ref(output)
    }

    pub fn header_unchecked<Header: bytemuck::Pod>(&self) -> &'p Header {
        let output = self
            .packet
            .sub_array_ref::<0, { pcp_packet::header::LEN }>();
        bytemuck::must_cast_ref(output)
    }

    pub fn opcode_unchecked<Data: bytemuck::Pod>(&self) -> &'p Data
    where
        [u8; size_of::<Data>()]:,
    {
        let output = self
            .packet
            .sub_array_ref::<{ pcp_packet::header::LEN }, { size_of::<Data>() }>();
        bytemuck::must_cast_ref(output)
    }

    pub fn map_request_data(&self) -> Option<(data::request::Header, data::request::Map)> {
        let pcp_packet::header::Request {
            meta,
            reserved1: _,
            requested_lifetime,
            client_ip_address,
        } = self.header_unchecked();
        if !check::meta(meta, false, pcp_consts::opcode::MAP) {
            return None;
        }

        let pcp_packet::opcode::map::Request {
            mapping_nonce,
            protocol,
            reserved1: _,
            internal_port,
            suggested_external_port,
            suggested_external_ip_address,
        } = self.opcode_unchecked();

        use pcp_primitives::{Address, LifetimeSeconds, Port};

        let header = data::request::Header {
            requested_lifetime: LifetimeSeconds::from_be_bytes(*requested_lifetime),
            client_ip_address: Address::from(*client_ip_address),
        };
        let data = data::request::Map {
            mapping_nonce: *mapping_nonce,
            protocol: *protocol,
            internal_port: Port::from_be_bytes(*internal_port),
            suggested_external_port: Port::from_be_bytes(*suggested_external_port),
            suggested_external_ip_address: Address::from(*suggested_external_ip_address),
        };

        Some((header, data))
    }

    pub fn map_response_data(&self) -> Option<(data::response::Header, data::response::Map)> {
        let pcp_packet::header::Response {
            meta,
            reserved1: _,
            result_code,
            lifetime,
            epoch_time,
            reserved2: _,
        } = self.header_unchecked();
        if !check::meta(meta, true, pcp_consts::opcode::MAP) {
            return None;
        }

        let pcp_packet::opcode::map::Response {
            mapping_nonce,
            protocol,
            reserved1: _,
            internal_port,
            assigned_external_port,
            assigned_external_ip_address,
        } = self.opcode_unchecked();

        use pcp_primitives::{Address, EpochTime, LifetimeSeconds, Port};

        let header = data::response::Header {
            result_code: *result_code,
            lifetime: LifetimeSeconds::from_be_bytes(*lifetime),
            epoch_time: EpochTime::from_be_bytes(*epoch_time),
        };
        let data = data::response::Map {
            mapping_nonce: *mapping_nonce,
            protocol: *protocol,
            internal_port: Port::from_be_bytes(*internal_port),
            assigned_external_port: Port::from_be_bytes(*assigned_external_port),
            assigned_external_ip_address: Address::from(*assigned_external_ip_address),
        };

        Some((header, data))
    }
}
