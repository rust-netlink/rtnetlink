// SPDX-License-Identifier: MIT

//! End-to-end validation of the panic-free emission chain:
//!
//! netlink-packet-core -> netlink-packet-route -> netlink-proto -> rtnetlink

use rtnetlink::{
    packet_core::{
        DefaultNla, EncodeErrorKind, NetlinkHeader, NetlinkMessage,
        NetlinkPayload, NLA_MAX_VALUE_LEN,
    },
    packet_route::{
        route::{RouteAttribute, RouteMessage},
        RouteNetlinkMessage,
    },
    proto::{NetlinkCodec, NetlinkMessageCodec},
};

#[test]
fn valid_route_message_round_trips_through_proto() {
    let message = NetlinkMessage::new(
        NetlinkHeader::default(),
        NetlinkPayload::InnerMessage(RouteNetlinkMessage::NewRoute(
            RouteMessage::default(),
        )),
    );

    let mut buffer = Vec::new();
    NetlinkCodec::encode(message.clone(), &mut buffer).unwrap();
    assert!(!buffer.is_empty());

    let mut expected_header = message.header;
    expected_header.length = buffer.len() as u32;
    expected_header.message_type = message.payload.message_type();
    let expected =
        NetlinkMessage::new(expected_header, message.payload.clone());

    let decoded = NetlinkCodec::decode::<RouteNetlinkMessage>(&mut buffer)
        .unwrap()
        .unwrap();
    assert_eq!(decoded, expected);
}

#[test]
fn oversized_route_attribute_returns_error_without_panic() {
    let mut route = RouteMessage::default();
    route.attributes.push(RouteAttribute::Other(DefaultNla::new(
        1,
        vec![0; NLA_MAX_VALUE_LEN + 1],
    )));
    let message = NetlinkMessage::new(
        NetlinkHeader::default(),
        NetlinkPayload::InnerMessage(RouteNetlinkMessage::NewRoute(route)),
    );

    let mut buffer = vec![0xaa];
    let error = NetlinkCodec::encode(message, &mut buffer).unwrap_err();
    assert_eq!(error.kind(), EncodeErrorKind::TooLarge);
    assert_eq!(buffer, vec![0xaa]);
}
