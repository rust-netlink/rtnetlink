// SPDX-License-Identifier: MIT

//! Guards the crate's public API surface.
//!
//! Every public type must be nameable through the crate root, so users never
//! have to depend on the layout of private modules.

// The imports below exist only to prove the names are reachable.
#![allow(unused_imports)]

#[test]
fn crate_root_types_are_nameable() {
    #[cfg(any(
        target_os = "linux",
        target_os = "fuchsia",
        target_os = "android"
    ))]
    use rtnetlink::LinkBridgeVlan;
    use rtnetlink::{
        AddrLabelAddRequest, AddrLabelDelRequest, AddrLabelGetRequest,
        AddrLabelHandle, AddrLabelMessageBuilder, AddressAddRequest,
        AddressDelRequest, AddressGetRequest, AddressHandle,
        AddressMessageBuilder, AfstatsRequest, Error, Handle,
        InvalidRouteMessage, IpVersion, LinkAddRequest, LinkAmt,
        LinkAssignNsidHandle, LinkBareudp, LinkBatAdv, LinkBond, LinkBondPort,
        LinkBridge, LinkBridgePort, LinkCan, LinkDelPropRequest,
        LinkDelRequest, LinkDsa, LinkDummy, LinkGeneve, LinkGetNsidHandle,
        LinkGetRequest, LinkGre, LinkGre6, LinkGtp, LinkHandle, LinkHsr,
        LinkIfb, LinkIp6Tnl, LinkIpIp, LinkIpVlan, LinkIpVtap, LinkIpoib,
        LinkMacSec, LinkMacVlan, LinkMacVtap, LinkMessageBuilder,
        LinkNetdevsim, LinkNetkit, LinkNewPropRequest, LinkNlmon, LinkPfcp,
        LinkRmNet, LinkSetRequest, LinkSit, LinkTeam, LinkTun, LinkUnspec,
        LinkVcan, LinkVeth, LinkVirtWifi, LinkVlan, LinkVrf, LinkVti, LinkVti6,
        LinkVxcan, LinkVxlan, LinkWireguard, LinkWwan, LinkXfrm,
        MulticastGroup, NeighbourAddRequest, NeighbourDelRequest,
        NeighbourGetRequest, NeighbourHandle, NeighbourMessageBuilder,
        QosMapping, RouteAddRequest, RouteDelRequest, RouteGetRequest,
        RouteHandle, RouteMessageBuilder, RouteNextHopBuilder, RuleAddRequest,
        RuleDelRequest, RuleGetRequest, RuleHandle, RuleMessageBuilder,
    };
    #[cfg(not(target_os = "freebsd"))]
    use rtnetlink::{
        NetworkNamespace, QDiscDelRequest, QDiscGetRequest, QDiscHandle,
        QDiscNewRequest, TrafficChainGetRequest, TrafficChainHandle,
        TrafficClassGetRequest, TrafficClassHandle, TrafficFilterDelRequest,
        TrafficFilterGetRequest, TrafficFilterHandle, TrafficFilterNewRequest,
    };
}

#[test]
fn internal_modules_are_not_exposed_as_pub_mod() {
    // Exposing an internal module would give its types a second, unstable
    // path such as `rtnetlink::link::LinkHandle`.
    let public_modules: Vec<&str> = include_str!("../src/lib.rs")
        .lines()
        .map(str::trim)
        .filter_map(|line| line.strip_prefix("pub mod "))
        .map(|module| module.trim_end_matches(';').trim())
        .collect();

    assert!(
        public_modules.is_empty(),
        "internal modules must stay private; re-export public items from the \
         crate root instead, found: {public_modules:?}"
    );
}
