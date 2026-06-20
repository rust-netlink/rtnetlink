// SPDX-License-Identifier: MIT

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::{
    packet_route::{
        link::{
            InfoData, InfoIpTunnel, InfoKind, TunnelEncapFlags, TunnelEncapType,
        },
        IpProtocol,
    },
    LinkMessageBuilder,
};

const SIT_ISATAP: u16 = 0x0001;

/// Represent SIT interface.
/// Example code on creating a SIT interface
/// ```no_run
/// use std::net::Ipv4Addr;
/// use rtnetlink::{new_connection, LinkSit};
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let (connection, handle, _) = new_connection().unwrap();
///     tokio::spawn(connection);
///
///     handle
///         .link()
///         .add(
///             LinkSit::new("sit0")
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
/// Please check LinkMessageBuilder::<LinkSit> for more detail.
#[derive(Default, Debug)]
pub struct LinkSit {
    flags: u16,
}

impl LinkSit {
    /// Equal to `LinkMessageBuilder::<LinkSit>::new()`
    pub fn new(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkSit>::new(name)
    }

    pub(crate) fn pre_build_info_data_process(
        &self,
        info_data: &mut Option<InfoData>,
    ) {
        if self.flags == 0 {
            return;
        }
        let InfoData::IpTunnel(infos) =
            info_data.get_or_insert_with(|| InfoData::IpTunnel(Vec::new()))
        else {
            log::error!("BUG: InfoData is not IpTunnel when processing sit");
            return;
        };
        infos.push(InfoIpTunnel::Ipv6SitFlags(self.flags));
    }
}

impl LinkMessageBuilder<LinkSit> {
    /// Create [LinkMessageBuilder] for SIT
    pub fn new(name: &str) -> Self {
        let mut builder =
            LinkMessageBuilder::<LinkSit>::new_with_info_kind(InfoKind::SitTun)
                .name(name.to_string());
        builder
            .set_pre_build_info_data_func(LinkSit::pre_build_info_data_process);
        builder
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
    /// `ip link add name NAME type sit local ADDR`.
    pub fn local(self, addr: Ipv4Addr) -> Self {
        self.append_info_data(InfoIpTunnel::Local(IpAddr::V4(addr)))
    }

    /// This is equivalent to `remote ADDR` in command
    /// `ip link add name NAME type sit remote ADDR`.
    pub fn remote(self, addr: Ipv4Addr) -> Self {
        self.append_info_data(InfoIpTunnel::Remote(IpAddr::V4(addr)))
    }

    /// This is equivalent to `ttl TTL` in command
    /// `ip link add name NAME type sit ttl TTL`.
    pub fn ttl(self, ttl: u8) -> Self {
        self.append_info_data(InfoIpTunnel::Ttl(ttl))
    }

    /// This is equivalent to `tos TOS` in command
    /// `ip link add name NAME type sit tos TOS`.
    pub fn tos(self, tos: u8) -> Self {
        self.append_info_data(InfoIpTunnel::Tos(tos))
    }

    /// This is equivalent to `mode MODE` in command
    /// `ip link add name NAME type sit mode MODE`.
    pub fn protocol(self, proto: IpProtocol) -> Self {
        self.append_info_data(InfoIpTunnel::Protocol(proto))
    }

    /// This is equivalent to `[no]pmtudisc` in command
    /// `ip link add name NAME type sit pmtudisc`.
    pub fn pmtudisc(self, enabled: bool) -> Self {
        self.append_info_data(InfoIpTunnel::PMtuDisc(enabled))
    }

    /// This is equivalent to `isatap` in command
    /// `ip link add name NAME type sit isatap`.
    pub fn isatap(mut self, enabled: bool) -> Self {
        if enabled {
            self.iface_self.flags |= SIT_ISATAP;
        } else {
            self.iface_self.flags &= !SIT_ISATAP;
        }
        self
    }

    /// This is equivalent to `6rd-prefix ADDR` in command
    /// `ip link add name NAME type sit 6rd-prefix ADDR`.
    pub fn ipv6_rd_prefix(self, addr: Ipv6Addr) -> Self {
        self.append_info_data(InfoIpTunnel::Ipv6RdPrefix(addr))
    }

    /// This is equivalent to `6rd-prefix ADDR/PREFIXLEN` in command
    /// `ip link add name NAME type sit 6rd-prefix ADDR/PREFIXLEN`.
    pub fn ipv6_rd_prefixlen(self, len: u16) -> Self {
        self.append_info_data(InfoIpTunnel::Ipv6RdPrefixLen(len))
    }

    /// This is equivalent to `6rd-relay_prefix ADDR` in command
    /// `ip link add name NAME type sit 6rd-relay_prefix ADDR`.
    pub fn ipv6_rd_relay_prefix(self, addr: Ipv4Addr) -> Self {
        self.append_info_data(InfoIpTunnel::Ipv6RdRelayPrefix(addr))
    }

    /// This is equivalent to `6rd-relay_prefix ADDR/PREFIXLEN` in command
    /// `ip link add name NAME type sit 6rd-relay_prefix ADDR/PREFIXLEN`.
    pub fn ipv6_rd_relay_prefixlen(self, len: u16) -> Self {
        self.append_info_data(InfoIpTunnel::Ipv6RdRelayPrefixLen(len))
    }

    /// This is equivalent to `dev PHYS_DEV` in command
    /// `ip link add name NAME type sit dev PHYS_DEV`.
    pub fn dev(self, ifindex: u32) -> Self {
        self.append_info_data(InfoIpTunnel::Link(ifindex))
    }

    /// This is equivalent to `external` in command
    /// `ip link add name NAME type sit external`.
    pub fn collect_metadata(self, enabled: bool) -> Self {
        if enabled {
            self.append_info_data(InfoIpTunnel::CollectMetadata)
        } else {
            self
        }
    }

    /// This is equivalent to `fwmark MARK` in command
    /// `ip link add name NAME type sit fwmark MARK`.
    pub fn fwmark(self, mark: u32) -> Self {
        self.append_info_data(InfoIpTunnel::FwMark(mark))
    }

    /// This is equivalent to `encap { fou | gue | none }` in command
    /// `ip link add name NAME type sit encap TYPE`.
    pub fn encap_type(self, encap_type: TunnelEncapType) -> Self {
        self.append_info_data(InfoIpTunnel::EncapType(encap_type))
    }

    /// This is equivalent to `encap-sport PORT` in command
    /// `ip link add name NAME type sit encap-sport PORT`.
    pub fn encap_sport(self, port: u16) -> Self {
        self.append_info_data(InfoIpTunnel::EncapSPort(port))
    }

    /// This is equivalent to `encap-dport PORT` in command
    /// `ip link add name NAME type sit encap-dport PORT`.
    pub fn encap_dport(self, port: u16) -> Self {
        self.append_info_data(InfoIpTunnel::EncapDPort(port))
    }

    /// This is equivalent to `encap-csum` in command
    /// `ip link add name NAME type sit encap-csum`.
    pub fn encap_flags(self, flags: TunnelEncapFlags) -> Self {
        self.append_info_data(InfoIpTunnel::EncapFlags(flags))
    }
}
