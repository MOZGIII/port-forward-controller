//! The type to encode the 7-bit `OPCODE` and the `R` bit in one `u8`.

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "bytemuck",
    derive(bytemuck::Pod, bytemuck::Zeroable, bytemuck::TransparentWrapper)
)]
#[repr(transparent)]
#[transparent(u8)]
pub struct RAndOpcode(pub u8);

const IS_RESPONSE_BIT: u8 = 0b1000_0000;

impl RAndOpcode {
    pub fn from_parts(is_response: bool, opcode: u8) -> Option<Self> {
        if opcode >= IS_RESPONSE_BIT {
            return None;
        }

        let mut val = opcode;
        if is_response {
            val |= IS_RESPONSE_BIT;
        }

        Some(Self(val))
    }

    pub fn opcode(self) -> u8 {
        self.0 & !IS_RESPONSE_BIT
    }

    pub fn is_response(self) -> bool {
        self.0 >= IS_RESPONSE_BIT
    }
}

impl core::fmt::Debug for RAndOpcode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let r: u8 = self.is_response().into();
        write!(f, "[R={}; OPCODE={}]", r, self.opcode())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    extern crate alloc;

    #[test]
    fn creation() {
        let cases = [
            (RAndOpcode::from_parts(false, 0), 0b0000_0000),
            (RAndOpcode::from_parts(true, 0), 0b1000_0000),
            //
            (RAndOpcode::from_parts(false, 1), 0b0000_0001),
            (RAndOpcode::from_parts(true, 1), 0b1000_0001),
            //
            (RAndOpcode::from_parts(false, 2), 0b0000_0010),
            (RAndOpcode::from_parts(false, 3), 0b0000_0011),
            (RAndOpcode::from_parts(true, 2), 0b1000_0010),
            (RAndOpcode::from_parts(true, 3), 0b1000_0011),
        ];

        for (actual, expected) in cases {
            assert_eq!(actual.unwrap().0, expected);
        }
    }

    #[test]
    fn debug() {
        let cases = [
            (RAndOpcode::from_parts(false, 0), "[R=0; OPCODE=0]"),
            (RAndOpcode::from_parts(true, 0), "[R=1; OPCODE=0]"),
            //
            (RAndOpcode::from_parts(false, 1), "[R=0; OPCODE=1]"),
            (RAndOpcode::from_parts(true, 1), "[R=1; OPCODE=1]"),
            //
            (RAndOpcode::from_parts(false, 2), "[R=0; OPCODE=2]"),
            (RAndOpcode::from_parts(false, 3), "[R=0; OPCODE=3]"),
            (RAndOpcode::from_parts(true, 2), "[R=1; OPCODE=2]"),
            (RAndOpcode::from_parts(true, 3), "[R=1; OPCODE=3]"),
        ];

        for (actual, expected) in cases {
            assert_eq!(alloc::format!("{:?}", actual.unwrap()), expected);
        }
    }
}
