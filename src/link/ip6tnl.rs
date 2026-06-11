// SPDX-License-Identifier: MIT

use std::net::Ipv6Addr;

use crate::{
    packet_route::{
        link::{
            InfoData, InfoIpTunnel, InfoKind, Ip6TunnelFlags, TunnelEncapFlags,
            TunnelEncapType,
        },
        IpProtocol,
    },
    LinkMessageBuilder,
};

/// Represent IP6TNL interface.
/// Example code on creating a IP6TNL interface
/// ```no_run
/// use std::net::Ipv6Addr;
/// use rtnetlink::{new_connection, LinkIp6Tnl};
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let (connection, handle, _) = new_connection().unwrap();
///     tokio::spawn(connection);
///
///     handle
///         .link()
///         .add(
///             LinkIp6Tnl::new("ip6tnl0")
///                 .local(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1))
///                 .remote(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 2))
///                 .build(),
///         )
///         .execute()
///         .await
///         .map_err(|e| format!("{e}"))
/// }
/// ```
///
/// Please check LinkMessageBuilder::<LinkIp6Tnl> for more detail.
#[derive(Debug)]
pub struct LinkIp6Tnl;

impl LinkIp6Tnl {
    /// Equal to `LinkMessageBuilder::<LinkIp6Tnl>::new()`
    pub fn new(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkIp6Tnl>::new(name)
    }
}

impl LinkMessageBuilder<LinkIp6Tnl> {
    /// Create [LinkMessageBuilder] for IP6TNL
    pub fn new(name: &str) -> Self {
        LinkMessageBuilder::<LinkIp6Tnl>::new_with_info_kind(InfoKind::Ip6Tnl)
            .name(name.to_string())
    }

    fn append_info_data(self, info: InfoIpTunnel) -> Self {
        let mut ret = self;
        if let InfoData::IpTunnel(infos) = ret
            .info_data
            .get_or_insert_with(|| InfoData::IpTunnel(Vec::new()))
        {
            infos.push(info);
        }
        ret
    }

    /// This is equivalent to `local ADDR` in command
    /// `ip link add name NAME type ip6tnl local ADDR`.
    pub fn local(self, addr: Ipv6Addr) -> Self {
        self.append_info_data(InfoIpTunnel::Local(addr.into()))
    }

    /// This is equivalent to `remote ADDR` in command
    /// `ip link add name NAME type ip6tnl remote ADDR`.
    pub fn remote(self, addr: Ipv6Addr) -> Self {
        self.append_info_data(InfoIpTunnel::Remote(addr.into()))
    }

    /// This is equivalent to `ttl TTL` in command
    /// `ip link add name NAME type ip6tnl ttl TTL`.
    pub fn ttl(self, ttl: u8) -> Self {
        self.append_info_data(InfoIpTunnel::Ttl(ttl))
    }

    /// This is equivalent to `tos TOS` in command
    /// `ip link add name NAME type ip6tnl tos TOS`.
    pub fn tos(self, tos: u8) -> Self {
        self.append_info_data(InfoIpTunnel::Tos(tos))
    }

    /// This is equivalent to `encaplimit LIMIT` in command
    /// `ip link add name NAME type ip6tnl encaplimit LIMIT`.
    pub fn encap_limit(self, limit: u8) -> Self {
        self.append_info_data(InfoIpTunnel::EncapLimit(limit))
    }

    /// This is equivalent to `flowlabel LABEL` in command
    /// `ip link add name NAME type ip6tnl flowlabel LABEL`.
    pub fn flowlabel(self, flowlabel: u32) -> Self {
        self.append_info_data(InfoIpTunnel::FlowInfo(flowlabel))
    }

    /// This is equivalent to `mode MODE` in command
    /// `ip link add name NAME type ip6tnl mode MODE`.
    pub fn protocol(self, proto: IpProtocol) -> Self {
        self.append_info_data(InfoIpTunnel::Protocol(proto))
    }

    /// This is equivalent to `[no]pmtudisc` in command
    /// `ip link add name NAME type ip6tnl pmtudisc`.
    pub fn pmtudisc(self, enabled: bool) -> Self {
        self.append_info_data(InfoIpTunnel::PMtuDisc(enabled))
    }

    /// This is equivalent to `dev PHYS_DEV` in command
    /// `ip link add name NAME type ip6tnl dev PHYS_DEV`.
    pub fn dev(self, ifindex: u32) -> Self {
        self.append_info_data(InfoIpTunnel::Link(ifindex))
    }

    /// This is equivalent to `external` in command
    /// `ip link add name NAME type ip6tnl external`.
    pub fn collect_metadata(self, enabled: bool) -> Self {
        if enabled {
            self.append_info_data(InfoIpTunnel::CollectMetadata)
        } else {
            self
        }
    }

    /// This is equivalent to `fwmark MARK` in command
    /// `ip link add name NAME type ip6tnl fwmark MARK`.
    /// Setting 0 means `inherit`.
    pub fn fwmark(self, mark: u32) -> Self {
        self.append_info_data(InfoIpTunnel::FwMark(mark))
    }

    /// Set IP6 tunnel flags
    pub fn ip6_flags(self, flags: Ip6TunnelFlags) -> Self {
        self.append_info_data(InfoIpTunnel::Ipv6Flags(flags))
    }

    /// This is equivalent to `encap { fou | gue | none }` in command
    /// `ip link add name NAME type ip6tnl encap TYPE`.
    pub fn encap_type(self, encap_type: TunnelEncapType) -> Self {
        self.append_info_data(InfoIpTunnel::EncapType(encap_type))
    }

    /// This is equivalent to `encap-sport PORT` in command
    /// `ip link add name NAME type ip6tnl encap-sport PORT`.
    pub fn encap_sport(self, port: u16) -> Self {
        self.append_info_data(InfoIpTunnel::EncapSPort(port))
    }

    /// This is equivalent to `encap-dport PORT` in command
    /// `ip link add name NAME type ip6tnl encap-dport PORT`.
    pub fn encap_dport(self, port: u16) -> Self {
        self.append_info_data(InfoIpTunnel::EncapDPort(port))
    }

    /// This is equivalent to `encap-csum` in command
    /// `ip link add name NAME type ip6tnl encap-csum`.
    pub fn encap_flags(self, flags: TunnelEncapFlags) -> Self {
        self.append_info_data(InfoIpTunnel::EncapFlags(flags))
    }
}
