use core::borrow::BorrowMut;

use const_sub_array::SubArray;

use crate::data;

use super::{steps, State};

impl<Packet: BorrowMut<pcp_packet::Buffer>> State<Packet, steps::NeedsResponseOpcode> {
    #[allow(clippy::result_large_err)]
    pub fn opcode<const OPCODE_DATA_LEN: usize>(
        self,
        response_header: data::response::Header,
        opcode: pcp_primitives::Opcode,
        opcode_data: &[u8; OPCODE_DATA_LEN],
    ) -> Result<State<Packet, steps::NeedsOptions<OPCODE_DATA_LEN>>, Self> {
        let Some(r_and_opcode) = pcp_packet::RAndOpcode::from_parts(true, opcode) else {
            return Err(self);
        };

        let data::response::Header {
            result_code,
            lifetime,
            epoch_time,
        } = response_header;

        let Self {
            step: steps::NeedsResponseOpcode,
            mut packet,
        } = self;

        {
            let packet = packet.borrow_mut();

            {
                let header: &mut pcp_packet::header::Buffer =
                    packet.sub_array_mut::<0, { pcp_packet::header::LEN }>();

                let header: &mut pcp_packet::header::Response = bytemuck::must_cast_mut(header);

                *header = pcp_packet::header::Response {
                    meta: pcp_packet::Meta {
                        version: pcp_consts::VERSION,
                        r_and_opcode,
                    },
                    reserved1: [0; 1],
                    result_code,
                    lifetime: lifetime.to_be_bytes(),
                    epoch_time: epoch_time.to_be_bytes(),
                    reserved2: [0; 12],
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
        response_header: data::response::Header,
        response_data: data::response::Map,
    ) -> State<Packet, steps::NeedsOptions<{ pcp_packet::opcode::map::LEN }>> {
        let data::response::Map {
            mapping_nonce,
            protocol,
            internal_port,
            assigned_external_port,
            assigned_external_ip_address,
        } = response_data;

        let opcode = pcp_consts::opcode::MAP;
        let opcode_data = pcp_packet::opcode::map::Response {
            mapping_nonce,
            protocol,
            reserved1: [0; 3],
            internal_port: internal_port.to_be_bytes(),
            assigned_external_port: assigned_external_port.to_be_bytes(),
            assigned_external_ip_address: assigned_external_ip_address.octets(),
        };
        let opcode_data_ref: &pcp_packet::opcode::map::Buffer =
            bytemuck::must_cast_ref(&opcode_data);
        self.opcode(response_header, opcode, opcode_data_ref)
            .unwrap()
    }
}
