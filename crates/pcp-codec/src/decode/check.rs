use pcp_primitives::Opcode;

pub fn meta(meta: &pcp_packet::Meta, expected_is_response: bool, expected_opcode: Opcode) -> bool {
    meta.version == pcp_consts::VERSION
        && meta.r_and_opcode.is_response() == expected_is_response
        && meta.r_and_opcode.opcode() == expected_opcode
}
