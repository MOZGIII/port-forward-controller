//! Request header.

//  0                   1                   2                   3
//  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |  Version = 2  |R|   Opcode    |         Reserved              | 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                 Requested Lifetime (32 bits)                  | 2
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                                                               | 3
// |            PCP Client's IP Address (128 bits)                 | 4
// |                                                               | 5
// |                                                               | 6
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// :                                                               :
// :             (optional) Opcode-specific information            :
// :                                                               :
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// :                                                               :
// :             (optional) PCP Options                            :
// :                                                               :
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

use crate::Meta;

static_assertions::assert_eq_size!(Data, super::Buffer);
static_assertions::assert_eq_align!(Data, super::Buffer);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C, packed)]
pub struct Data {
    pub meta: Meta,
    pub reserved1: [u8; 2],
    pub requested_lifetime: [u8; 4],
    pub client_ip_address: [u8; 16],
}
