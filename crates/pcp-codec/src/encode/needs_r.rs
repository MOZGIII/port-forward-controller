use core::borrow::BorrowMut;

use super::{steps, State};

impl State<pcp_packet::Buffer, steps::NeedsR> {
    pub fn new_owned() -> Self {
        Self::from_zeroed_unchecked([0; pcp_packet::LEN])
    }
}

impl<Packet: BorrowMut<pcp_packet::Buffer>> State<Packet, steps::NeedsR> {
    pub fn new(mut packet: Packet) -> Self {
        *packet.borrow_mut() = [0; pcp_packet::LEN];
        Self::from_zeroed_unchecked(packet)
    }

    pub fn from_zeroed_unchecked(packet: Packet) -> Self {
        Self {
            step: steps::NeedsR,
            packet,
        }
    }

    pub fn request(self) -> State<Packet, steps::NeedsRequestOpcode> {
        let Self {
            step: steps::NeedsR,
            packet,
        } = self;

        State {
            step: steps::NeedsRequestOpcode,
            packet,
        }
    }

    pub fn response(self) -> State<Packet, steps::NeedsResponseOpcode> {
        let Self {
            step: steps::NeedsR,
            packet,
        } = self;

        State {
            step: steps::NeedsResponseOpcode,
            packet,
        }
    }
}
