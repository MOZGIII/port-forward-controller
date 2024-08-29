use core::borrow::BorrowMut;

use const_sub_array::SubArray;

use super::{steps, State};

impl<Packet: BorrowMut<pcp_packet::Buffer>, const NEXT_OPTION_OFFSET: usize>
    State<Packet, steps::NeedsOptions<NEXT_OPTION_OFFSET>>
{
    #[allow(clippy::result_large_err)]
    pub fn add_option<const OPTION_DATA_LEN: usize>(
        self,
        option_code: u8,
        option_data: &[u8; OPTION_DATA_LEN],
    ) -> State<
        Packet,
        steps::NeedsOptions<
            { NEXT_OPTION_OFFSET + pcp_packet::option::header::LEN + OPTION_DATA_LEN },
        >,
    > {
        let Self {
            step: steps::NeedsOptions,
            mut packet,
        } = self;

        {
            let packet = packet.borrow_mut();
            {
                let header: &mut pcp_packet::option::header::Buffer = packet
                    .sub_array_mut::<NEXT_OPTION_OFFSET, { pcp_packet::option::header::LEN }>();

                let header: &mut pcp_packet::option::header::Data = bytemuck::must_cast_mut(header);

                *header = pcp_packet::option::header::Data {
                    option_code,
                    reserved1: [0; 1],
                    option_length: (OPTION_DATA_LEN as u16).to_be_bytes(),
                };
            }

            {
                let data: &mut [u8; OPTION_DATA_LEN]  = packet.sub_array_mut::<{ NEXT_OPTION_OFFSET + pcp_packet::option::header::LEN }, OPTION_DATA_LEN>();
                *data = *option_data;
            }
        }

        State {
            step: steps::NeedsOptions,
            packet,
        }
    }

    pub fn finish(self) -> Packet {
        self.packet
    }
}
