// SPDX-License-Identifier: MIT

mod add;
mod afstats;
mod amt;
mod bareudp;
mod batadv;
mod bond;
mod bond_port;
mod bridge;
mod bridge_port;
#[cfg(any(target_os = "linux", target_os = "fuchsia", target_os = "android"))]
mod bridge_vlan;
mod builder;
mod can;
mod del;
mod dsa;
mod dummy;
mod geneve;
mod get;
mod gre;
mod gre6;
mod gtp;
mod handle;
mod hsr;
mod ifb;
mod ip6tnl;
mod iptun;
mod ipvlan;
mod mac_vlan;
mod mac_vtap;
mod macsec;
mod netdevsim;
mod netkit;
mod nlmon;
mod pfcp;
mod property_add;
mod property_del;
mod rmnet;
mod set;
mod sit;
mod team;
mod vcan;
mod veth;
mod virt_wifi;
mod vlan;
mod vrf;
mod vti;
mod vti6;
mod vxcan;
mod vxlan;
mod wireguard;
mod wwan;
mod xfrm;

pub use self::{
    add::LinkAddRequest,
    afstats::AfstatsRequest,
    amt::LinkAmt,
    bareudp::LinkBareudp,
    batadv::LinkBatAdv,
    bond::LinkBond,
    bond_port::LinkBondPort,
    bridge::LinkBridge,
    bridge_port::LinkBridgePort,
    builder::{LinkMessageBuilder, LinkUnspec},
    can::LinkCan,
    del::LinkDelRequest,
    dsa::LinkDsa,
    dummy::LinkDummy,
    geneve::LinkGeneve,
    get::LinkGetRequest,
    gre::LinkGre,
    gre6::LinkGre6,
    gtp::LinkGtp,
    handle::LinkHandle,
    hsr::LinkHsr,
    ifb::LinkIfb,
    ip6tnl::LinkIp6Tnl,
    iptun::LinkIpIp,
    ipvlan::{LinkIpVlan, LinkIpVtap},
    mac_vlan::LinkMacVlan,
    mac_vtap::LinkMacVtap,
    macsec::LinkMacSec,
    netdevsim::LinkNetdevsim,
    netkit::LinkNetkit,
    nlmon::LinkNlmon,
    pfcp::LinkPfcp,
    property_add::LinkNewPropRequest,
    property_del::LinkDelPropRequest,
    rmnet::LinkRmNet,
    set::LinkSetRequest,
    sit::LinkSit,
    team::LinkTeam,
    vcan::LinkVcan,
    veth::LinkVeth,
    virt_wifi::LinkVirtWifi,
    vlan::{LinkVlan, QosMapping},
    vrf::LinkVrf,
    vti::LinkVti,
    vti6::LinkVti6,
    vxcan::LinkVxcan,
    vxlan::LinkVxlan,
    wireguard::LinkWireguard,
    wwan::LinkWwan,
    xfrm::LinkXfrm,
};

#[cfg(test)]
mod test;

#[cfg(any(
    target_os = "linux",
    target_os = "fuchsia",
    target_os = "android"
))]
pub use self::bridge_vlan::LinkBridgeVlan;
