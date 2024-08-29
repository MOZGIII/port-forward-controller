//! `MAP` request.

//  0                   1                   2                   3
//  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                                                               | 1
// |                 Mapping Nonce (96 bits)                       | 2
// |                                                               | 3
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |   Protocol    |          Reserved (24 bits)                   | 4
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |        Internal Port          |    Suggested External Port    | 5
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                                                               | 6
// |           Suggested External IP Address (128 bits)            | 7
// |                                                               | 8
// |                                                               | 9
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

static_assertions::assert_eq_size!(Data, super::Buffer);
static_assertions::assert_eq_align!(Data, super::Buffer);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C, packed)]
pub struct Data {
    pub mapping_nonce: [u8; 12],
    pub protocol: u8,
    pub reserved1: [u8; 3],
    pub internal_port: [u8; 2],
    pub suggested_external_port: [u8; 2],
    pub suggested_external_ip_address: [u8; 16],
}
