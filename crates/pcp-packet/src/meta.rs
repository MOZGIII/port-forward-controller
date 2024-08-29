use crate::RAndOpcode;

/// Version, message direction (aka `R`) and opcode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C, packed)]
pub struct Meta {
    pub version: u8,
    pub r_and_opcode: RAndOpcode,
}
