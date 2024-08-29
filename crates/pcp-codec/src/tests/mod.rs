use core::net::{Ipv4Addr, Ipv6Addr};

use crate::{data::request, decode, encode};

fn assert_packet<const ASSERTION_LEN: usize>(
    packet: pcp_packet::Buffer,
    assertion: [u8; ASSERTION_LEN],
) {
    assert_eq!(&packet[..ASSERTION_LEN], assertion);
    assert!(&packet[ASSERTION_LEN..].iter().all(|&byte| byte == 0));
}

#[test]
fn encode() {
    let packet = encode::State::new_owned()
        .request()
        .map(
            request::Header {
                requested_lifetime: 60,
                client_ip_address: Ipv4Addr::new(1, 2, 3, 4).to_ipv6_mapped(),
            },
            request::Map {
                mapping_nonce: [
                    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
                ],
                protocol: pcp_consts::protocol::TCP,
                internal_port: 80,
                suggested_external_port: 80,
                suggested_external_ip_address: Ipv6Addr::UNSPECIFIED,
            },
        )
        .finish();

    let expected = [
        0x02, // version
        0x01, // r and opcode for MAP request
        0, 0, // reserved, zeroes
        0, 0, 0, 60, // lifetime, 60 seconds
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, 1, 2, 3, 4, // client IP address
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, // nonce
        6,    // protocol, TCP
        0, 0, 0, // reserved, zeroes
        0, 80, // internal port
        0, 80, // suggested external port
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // suggested external IP address
    ];

    assert_packet(packet, expected);
}

#[test]
fn decode() {
    let sample_header: request::Header = request::Header {
        requested_lifetime: 60,
        client_ip_address: Ipv4Addr::new(1, 2, 3, 4).to_ipv6_mapped(),
    };

    let sample_map: request::Map = request::Map {
        mapping_nonce: [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
        ],
        protocol: pcp_consts::protocol::TCP,
        internal_port: 80,
        suggested_external_port: 80,
        suggested_external_ip_address: Ipv6Addr::UNSPECIFIED,
    };

    let packet = encode::State::new_owned()
        .request()
        .map(sample_header, sample_map)
        .finish();

    let decoder = decode::State::new(&packet);

    assert_eq!(
        decoder.map_request_data().unwrap(),
        (sample_header, sample_map)
    );
}
