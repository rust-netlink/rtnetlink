// SPDX-License-Identifier: MIT

use std::net::Ipv4Addr;

use rtnetlink::{
    packet_core::{DefaultNla, NetlinkMessage, NetlinkPayload},
    packet_route::{
        link::{InfoData, InfoKind, LinkAttribute, LinkInfo, LinkMessage},
        route::{RouteAttribute, RouteMessage},
        RouteNetlinkMessage,
    },
    LinkMessageBuilder, LinkUnspec, MulticastGroup, RouteMessageBuilder,
};

#[test]
fn link_builder_emits_arbitrary_attribute() {
    let attribute =
        LinkAttribute::Other(DefaultNla::new(0x7fff, vec![1, 2, 3]));
    let message: LinkMessage = LinkUnspec::new_with_name("future0")
        .append_extra_attribute(attribute.clone())
        .build();

    assert!(message.attributes.contains(&attribute));
}

#[test]
fn link_builder_emits_arbitrary_info_kind_and_data() {
    let message = LinkMessageBuilder::<LinkUnspec>::new_with_info_kind(
        InfoKind::Other("future-kind".to_string()),
    )
    .name("future0")
    .set_info_data(InfoData::Other(vec![0xde, 0xad, 0xbe, 0xef]))
    .build();

    let infos = message
        .attributes
        .iter()
        .find_map(|attribute| match attribute {
            LinkAttribute::LinkInfo(infos) => Some(infos),
            _ => None,
        })
        .expect("link info attribute is missing");

    assert!(infos.iter().any(|info| matches!(
        info,
        LinkInfo::Kind(InfoKind::Other(kind)) if kind == "future-kind"
    )));
    assert!(infos.iter().any(|info| matches!(
        info,
        LinkInfo::Data(InfoData::Other(data))
            if data == &vec![0xde, 0xad, 0xbe, 0xef]
    )));
}

#[test]
fn route_builder_emits_arbitrary_attribute() {
    let attribute =
        RouteAttribute::Other(DefaultNla::new(0x7ffe, vec![4, 5, 6]));
    let mut builder = RouteMessageBuilder::<Ipv4Addr>::new();
    builder.get_mut().attributes.push(attribute.clone());
    let message: RouteMessage = builder.build();

    assert!(message.attributes.contains(&attribute));
}

#[test]
fn exported_macros_resolve_through_rtnetlink() -> Result<(), rtnetlink::Error> {
    let message = || {
        Ok::<_, rtnetlink::Error>(NetlinkMessage::from(
            RouteNetlinkMessage::NewRoute(RouteMessage::default()),
        ))
    };

    let parsed = rtnetlink::try_rtnl!(message(), RouteNetlinkMessage::NewRoute);
    assert_eq!(parsed.header, RouteMessage::default().header);

    rtnetlink::try_nl!(message());
    Ok(())
}

#[test]
fn multicast_group_covers_kernel_uapi_values() {
    let groups = [
        (MulticastGroup::Link, 1),
        (MulticastGroup::Notify, 2),
        (MulticastGroup::Neigh, 3),
        (MulticastGroup::Tc, 4),
        (MulticastGroup::Ipv4Ifaddr, 5),
        (MulticastGroup::Ipv4Mroute, 6),
        (MulticastGroup::Ipv4Route, 7),
        (MulticastGroup::Ipv4Rule, 8),
        (MulticastGroup::Ipv6Ifaddr, 9),
        (MulticastGroup::Ipv6Mroute, 10),
        (MulticastGroup::Ipv6Route, 11),
        (MulticastGroup::Ipv6Ifinfo, 12),
        (MulticastGroup::DecnetIfaddr, 13),
        (MulticastGroup::DecnetRoute, 15),
        (MulticastGroup::DecnetRule, 16),
        (MulticastGroup::Ipv6Prefix, 18),
        (MulticastGroup::Ipv6Rule, 19),
        (MulticastGroup::NdUseropt, 20),
        (MulticastGroup::PhonetIfaddr, 21),
        (MulticastGroup::PhonetRoute, 22),
        (MulticastGroup::Dcb, 23),
        (MulticastGroup::Ipv4Netconf, 24),
        (MulticastGroup::Ipv6Netconf, 25),
        (MulticastGroup::Mdb, 26),
        (MulticastGroup::MplsRoute, 27),
        (MulticastGroup::Nsid, 28),
        (MulticastGroup::MplsNetconf, 29),
        (MulticastGroup::Ipv4MrouteR, 30),
        (MulticastGroup::Ipv6MrouteR, 31),
        (MulticastGroup::Nexthop, 32),
        (MulticastGroup::Brvlan, 33),
        (MulticastGroup::MctpIfaddr, 34),
        (MulticastGroup::Tunnel, 35),
        (MulticastGroup::Stats, 36),
        (MulticastGroup::Ipv4Mcaddr, 37),
        (MulticastGroup::Ipv6Mcaddr, 38),
        (MulticastGroup::Ipv6Acaddr, 39),
    ];

    for (group, expected) in groups {
        assert_eq!(group as u32, expected, "wrong value for {group:?}");
    }

    let covered: Vec<u32> =
        groups.iter().map(|(group, _)| *group as u32).collect();
    for value in 1..=39 {
        if value == 14 || value == 17 {
            continue;
        }
        assert!(
            covered.contains(&value),
            "RTNLGRP value {value} is not covered"
        );
    }
}

#[test]
fn payload_type_is_usable_without_upstream_imports(
) -> Result<(), rtnetlink::Error> {
    let payload = NetlinkPayload::<RouteNetlinkMessage>::Noop;
    let message = NetlinkMessage::new(
        rtnetlink::packet_core::NetlinkHeader::default(),
        payload,
    );
    rtnetlink::try_nl!(Ok::<_, rtnetlink::Error>(message));
    Ok(())
}
