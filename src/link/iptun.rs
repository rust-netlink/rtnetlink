// SPDX-License-Identifier: MIT

use std::net::{IpAddr, Ipv4Addr};

use crate::{
    packet_route::{
        link::{InfoData, InfoIpTunnel, InfoKind},
        IpProtocol,
    },
    LinkMessageBuilder,
};

/// Represent IPIP interface.
/// Example code on creating a IPIP interface
/// ```no_run
/// use std::net::Ipv4Addr;
/// use rtnetlink::{new_connection, LinkIpIp};
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let (connection, handle, _) = new_connection().unwrap();
///     tokio::spawn(connection);
///
///     handle
///         .link()
///         .add(
///             LinkIpIp::new("ipip0")
///                 .local(Ipv4Addr::new(192, 168, 1, 1))
///                 .remote(Ipv4Addr::new(10, 0, 0, 1))
///                 .build(),
///         )
///         .execute()
///         .await
///         .map_err(|e| format!("{e}"))
/// }
/// ```
///
/// Please check LinkMessageBuilder::<LinkIpIp> for more detail.
#[derive(Debug)]
pub struct LinkIpIp;

impl LinkIpIp {
    /// Equal to `LinkMessageBuilder::<LinkIpIp>::new()`
    pub fn new(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkIpIp>::new(name)
    }
}

impl LinkMessageBuilder<LinkIpIp> {
    /// Create [LinkMessageBuilder] for IPIP
    pub fn new(name: &str) -> Self {
        LinkMessageBuilder::<LinkIpIp>::new_with_info_kind(InfoKind::IpIp)
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
    /// `ip link add name NAME type ipip local ADDR`.
    pub fn local(self, addr: Ipv4Addr) -> Self {
        self.append_info_data(InfoIpTunnel::Local(IpAddr::V4(addr)))
    }

    /// This is equivalent to `remote ADDR` in command
    /// `ip link add name NAME type ipip remote ADDR`.
    pub fn remote(self, addr: Ipv4Addr) -> Self {
        self.append_info_data(InfoIpTunnel::Remote(IpAddr::V4(addr)))
    }

    /// This is equivalent to `ttl TTL` in command
    /// `ip link add name NAME type ipip ttl TTL`.
    pub fn ttl(self, ttl: u8) -> Self {
        self.append_info_data(InfoIpTunnel::Ttl(ttl))
    }

    /// This is equivalent to `tos TOS` in command
    /// `ip link add name NAME type ipip tos TOS`.
    pub fn tos(self, tos: u8) -> Self {
        self.append_info_data(InfoIpTunnel::Tos(tos))
    }

    /// This is equivalent to `mode MODE` in command
    /// `ip link add name NAME type ipip mode MODE`.
    pub fn protocol(self, proto: IpProtocol) -> Self {
        self.append_info_data(InfoIpTunnel::Protocol(proto))
    }

    /// This is equivalent to `[no]pmtudisc` in command
    /// `ip link add name NAME type ipip pmtudisc`.
    pub fn pmtudisc(self, enabled: bool) -> Self {
        self.append_info_data(InfoIpTunnel::PMtuDisc(enabled))
    }

    /// This is equivalent to `dev PHYS_DEV` in command
    /// `ip link add name NAME type ipip dev PHYS_DEV`.
    pub fn dev(self, ifindex: u32) -> Self {
        self.append_info_data(InfoIpTunnel::Link(ifindex))
    }

    /// This is equivalent to `external` in command
    /// `ip link add name NAME type ipip external`.
    pub fn collect_metadata(self, enabled: bool) -> Self {
        self.append_info_data(InfoIpTunnel::CollectMetadata(enabled))
    }

    /// This is equivalent to `fwmark MARK` in command
    /// `ip link add name NAME type ipip fwmark MARK`.
    pub fn fwmark(self, mark: u32) -> Self {
        self.append_info_data(InfoIpTunnel::FwMark(mark))
    }

    /// This is equivalent to `encap { fou | gue | none }` in command
    /// `ip link add name NAME type ipip encap TYPE`.
    pub fn encap_type(
        self,
        encap_type: crate::packet_route::link::TunnelEncapType,
    ) -> Self {
        self.append_info_data(InfoIpTunnel::EncapType(encap_type))
    }

    /// This is equivalent to `encap-sport PORT` in command
    /// `ip link add name NAME type ipip encap-sport PORT`.
    pub fn encap_sport(self, port: u16) -> Self {
        self.append_info_data(InfoIpTunnel::EncapSPort(port))
    }

    /// This is equivalent to `encap-dport PORT` in command
    /// `ip link add name NAME type ipip encap-dport PORT`.
    pub fn encap_dport(self, port: u16) -> Self {
        self.append_info_data(InfoIpTunnel::EncapDPort(port))
    }

    /// This is equivalent to `encap-csum` in command
    /// `ip link add name NAME type ipip encap-csum`.
    pub fn encap_flags(
        self,
        flags: crate::packet_route::link::TunnelEncapFlags,
    ) -> Self {
        self.append_info_data(InfoIpTunnel::EncapFlags(flags))
    }
}
