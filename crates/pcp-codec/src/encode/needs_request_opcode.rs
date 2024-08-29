use core::borrow::BorrowMut;

use const_sub_array::SubArray;

use crate::data;

use super::{steps, State};

impl<Packet: BorrowMut<pcp_packet::Buffer>> State<Packet, steps::NeedsRequestOpcode> {
    #[allow(clippy::result_large_err)]
    pub fn opcode<const OPCODE_DATA_LEN: usize>(
        self,
        request_header: data::request::Header,
        opcode: pcp_primitives::Opcode,
        opcode_data: &[u8; OPCODE_DATA_LEN],
    ) -> Result<State<Packet, steps::NeedsOptions<OPCODE_DATA_LEN>>, Self> {
        let Some(r_and_opcode) = pcp_packet::RAndOpcode::from_parts(false, opcode) else {
            return Err(self);
        };

        let data::request::Header {
            requested_lifetime,
            client_ip_address,
        } = request_header;

        let Self {
            step: steps::NeedsRequestOpcode,
            mut packet,
        } = self;

        {
            let packet = packet.borrow_mut();

            {
                let header: &mut pcp_packet::header::Buffer =
                    packet.sub_array_mut::<0, { pcp_packet::header::LEN }>();

                let header: &mut pcp_packet::header::Request = bytemuck::must_cast_mut(header);

                *header = pcp_packet::header::Request {
                    meta: pcp_packet::Meta {
                        version: pcp_consts::VERSION,
                        r_and_opcode,
                    },
                    reserved1: [0; 2],
                    requested_lifetime: requested_lifetime.to_be_bytes(),
                    client_ip_address: client_ip_address.octets(),
                };
            }

            {
                let data: &mut [u8; OPCODE_DATA_LEN] =
                    packet.sub_array_mut::<{ pcp_packet::header::LEN }, { OPCODE_DATA_LEN }>();
                *data = *opcode_data;
            }
        }

        Ok(State {
            step: steps::NeedsOptions,
            packet,
        })
    }

    #[allow(clippy::result_large_err)]
    pub fn map(
        self,
        request_header: data::request::Header,
        request_data: data::request::Map,
    ) -> State<Packet, steps::NeedsOptions<{ pcp_packet::opcode::map::LEN }>> {
        let data::request::Map {
            mapping_nonce,
            protocol,
            internal_port,
            suggested_external_port,
            suggested_external_ip_address,
        } = request_data;

        let opcode = pcp_consts::opcode::MAP;
        let opcode_data = pcp_packet::opcode::map::Request {
            mapping_nonce,
            protocol,
            reserved1: [0; 3],
            internal_port: internal_port.to_be_bytes(),
            suggested_external_port: suggested_external_port.to_be_bytes(),
            suggested_external_ip_address: suggested_external_ip_address.octets(),
        };
        let opcode_data_ref: &pcp_packet::opcode::map::Buffer =
            bytemuck::must_cast_ref(&opcode_data);
        self.opcode(request_header, opcode, opcode_data_ref)
            .unwrap()
    }
}
