// SPDX-License-Identifier: MIT

//! This crate provides methods to manipulate networking resources (links,
//! addresses, arp tables, route tables) via the netlink protocol.
//!
//! # Upstream crate re-exports
//!
//! The [`packet_core`], [`packet_route`] and [`proto`] modules re-export the
//! upstream crates whose types are part of this crate's public API. `rtnetlink
//! 1.x` relies on the `1.x` semver releases of those crates; an upstream major
//! release requires a matching `rtnetlink` major release.

#![allow(clippy::module_inception)]

pub use netlink_packet_core as packet_core;
pub use netlink_packet_route as packet_route;
pub use netlink_proto as proto;
pub use netlink_proto::sys;

mod addr;
mod addrlabel;
mod connection;
mod errors;
mod handle;
mod link;
mod macros;
mod multicast;
mod neighbour;
#[cfg(not(target_os = "freebsd"))]
mod ns;
mod route;
mod rule;
#[cfg(not(target_os = "freebsd"))]
mod traffic_control;

#[cfg(feature = "tokio_socket")]
pub use crate::connection::{
    new_connection, new_connection_with_strict_check, new_multicast_connection,
};
#[cfg(any(
    target_os = "linux",
    target_os = "fuchsia",
    target_os = "android"
))]
pub use crate::link::LinkBridgeVlan;
#[cfg(not(target_os = "freebsd"))]
pub use crate::ns::{NetworkNamespace, NETNS_PATH, NONE_FS, SELF_NS_PATH};
#[cfg(not(target_os = "freebsd"))]
pub use crate::traffic_control::{
    QDiscDelRequest, QDiscGetRequest, QDiscHandle, QDiscNewRequest,
    TcMessageBuilder, TrafficChainGetRequest, TrafficChainHandle,
    TrafficClassGetRequest, TrafficClassHandle, TrafficFilterDelRequest,
    TrafficFilterGetRequest, TrafficFilterHandle, TrafficFilterNewRequest,
};
pub use crate::{
    addr::{
        AddressAddRequest, AddressDelRequest, AddressGetRequest, AddressHandle,
        AddressMessageBuilder,
    },
    addrlabel::{
        AddrLabelAddRequest, AddrLabelDelRequest, AddrLabelGetRequest,
        AddrLabelHandle, AddrLabelMessageBuilder,
    },
    connection::{
        from_socket, new_connection_with_socket,
        new_multicast_connection_with_socket, NetlinkMessageStream,
    },
    errors::Error,
    handle::Handle,
    link::{
        AfstatsRequest, LinkAddRequest, LinkAmt, LinkAssignNsidHandle,
        LinkBareudp, LinkBatAdv, LinkBond, LinkBondPort, LinkBridge,
        LinkBridgePort, LinkCan, LinkDelPropRequest, LinkDelRequest, LinkDsa,
        LinkDummy, LinkGeneve, LinkGetNsidHandle, LinkGetRequest, LinkGre,
        LinkGre6, LinkGtp, LinkHandle, LinkHsr, LinkIfb, LinkIp6Tnl, LinkIpIp,
        LinkIpVlan, LinkIpVtap, LinkIpoib, LinkMacSec, LinkMacVlan,
        LinkMacVtap, LinkMessageBuilder, LinkNetdevsim, LinkNetkit,
        LinkNewPropRequest, LinkNlmon, LinkPfcp, LinkRmNet, LinkSetRequest,
        LinkSit, LinkTeam, LinkTun, LinkUnspec, LinkVcan, LinkVeth,
        LinkVirtWifi, LinkVlan, LinkVrf, LinkVti, LinkVti6, LinkVxcan,
        LinkVxlan, LinkWireguard, LinkWwan, LinkXfrm, QosMapping,
    },
    multicast::MulticastGroup,
    neighbour::{
        NeighbourAddRequest, NeighbourDelRequest, NeighbourGetRequest,
        NeighbourHandle, NeighbourMessageBuilder,
    },
    route::{
        InvalidRouteMessage, IpVersion, RouteAddRequest, RouteDelRequest,
        RouteGetRequest, RouteHandle, RouteMessageBuilder, RouteNextHopBuilder,
    },
    rule::{
        RuleAddRequest, RuleDelRequest, RuleGetRequest, RuleHandle,
        RuleMessageBuilder,
    },
};
