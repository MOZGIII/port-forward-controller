//! Option.

//  0                   1                   2                   3
//  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |  Option Code  |  Reserved     |       Option Length           | 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// :                       (optional) Data                         :
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

static_assertions::assert_eq_size!(Data, Buffer);
static_assertions::assert_eq_align!(Data, Buffer);

pub const LEN: usize = crate::ROW_SIZE; // * 1

pub type Buffer = [u8; LEN];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C, packed)]
pub struct Data {
    pub option_code: u8,
    pub reserved1: [u8; 1],
    pub option_length: [u8; 2],
}
