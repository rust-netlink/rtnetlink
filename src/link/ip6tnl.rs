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

const IP6_FLOWINFO_TCLASS_MASK: u32 = 0x0ff00000;
const IP6_FLOWINFO_FLOWLABEL_MASK: u32 = 0x000fffff;

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
#[derive(Default, Debug)]
pub struct LinkIp6Tnl {
    flags: Option<Ip6TunnelFlags>,
    flowinfo: Option<u32>,
}

impl LinkIp6Tnl {
    /// Equal to `LinkMessageBuilder::<LinkIp6Tnl>::new()`
    pub fn new(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkIp6Tnl>::new(name)
    }

    fn get_flags_mut(&mut self) -> &mut Ip6TunnelFlags {
        self.flags.get_or_insert(Ip6TunnelFlags::empty())
    }

    fn get_flowinfo_mut(&mut self) -> &mut u32 {
        self.flowinfo.get_or_insert(0)
    }

    pub(crate) fn pre_build_info_data_process(
        &self,
        info_data: &mut Option<InfoData>,
    ) {
        if self.flowinfo.is_none() && self.flags.is_none() {
            return;
        }
        let InfoData::IpTunnel(infos) =
            info_data.get_or_insert_with(|| InfoData::IpTunnel(Vec::new()))
        else {
            log::error!("BUG: InfoData is not IpTunnel when processing ip6tnl");
            return;
        };
        if let Some(flowinfo) = self.flowinfo {
            infos.push(InfoIpTunnel::FlowInfo(flowinfo));
        }
        if let Some(flags) = self.flags {
            infos.push(InfoIpTunnel::Ipv6Flags(flags));
        }
    }
}

impl LinkMessageBuilder<LinkIp6Tnl> {
    /// Create [LinkMessageBuilder] for IP6TNL
    pub fn new(name: &str) -> Self {
        let mut builder = LinkMessageBuilder::<LinkIp6Tnl>::new_with_info_kind(
            InfoKind::Ip6Tnl,
        )
        .name(name.to_string());
        builder.set_pre_build_info_data_func(
            LinkIp6Tnl::pre_build_info_data_process,
        );
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

    /// This is equivalent to `encaplimit LIMIT` in command
    /// `ip link add name NAME type ip6tnl encaplimit LIMIT`.
    pub fn encap_limit(self, limit: u8) -> Self {
        self.append_info_data(InfoIpTunnel::EncapLimit(limit))
    }

    /// Set the traffic class value.
    ///
    /// `val` is the 8-bit traffic class value.
    /// When `Some`, the tclass bits are set and `UseOrigTclass` flag is
    /// cleared. When `None`, the `UseOrigTclass` flag is set (inherit).
    pub fn tclass(mut self, val: Option<u8>) -> Self {
        match val {
            Some(v) => {
                let flowinfo = self.iface_self.get_flowinfo_mut();
                *flowinfo = (*flowinfo & !IP6_FLOWINFO_TCLASS_MASK)
                    | ((v as u32) << 20);
                let f = self.iface_self.get_flags_mut();
                *f &= !Ip6TunnelFlags::UseOrigTclass;
            }
            None => {
                let f = self.iface_self.get_flags_mut();
                *f |= Ip6TunnelFlags::UseOrigTclass;
            }
        }
        self
    }

    /// Set the flow label value.
    ///
    /// `val` is the 20-bit flow label value.
    /// When `Some`, the flowlabel bits are set and `UseOrigFlowlabel` flag
    /// is cleared. When `None`, the `UseOrigFlowlabel` flag is set (inherit).
    /// Values exceeding 20 bits are ignored with a warning logged.
    pub fn flowlabel(mut self, val: Option<u32>) -> Self {
        match val {
            Some(v) => {
                if v > 0xfffff {
                    log::error!(
                        "flowlabel value {v:#x} exceeds 20 bits \
                        (max 0xFFFFF), ignoring"
                    );
                    return self;
                }
                let flowinfo = self.iface_self.get_flowinfo_mut();
                *flowinfo = (*flowinfo & !IP6_FLOWINFO_FLOWLABEL_MASK)
                    | (v & IP6_FLOWINFO_FLOWLABEL_MASK);
                let f = self.iface_self.get_flags_mut();
                *f &= !Ip6TunnelFlags::UseOrigFlowlabel;
            }
            None => {
                let f = self.iface_self.get_flags_mut();
                *f |= Ip6TunnelFlags::UseOrigFlowlabel;
            }
        }
        self
    }

    /// Set or clear an ip6 tunnel flag
    pub fn set_flag(mut self, flag: Ip6TunnelFlags, enabled: bool) -> Self {
        let f = self.iface_self.get_flags_mut();
        if enabled {
            *f |= flag;
        } else {
            *f &= !flag;
        }
        self
    }

    /// This is equivalent to `mode MODE` in command
    /// `ip link add name NAME type ip6tnl mode MODE`.
    pub fn protocol(self, proto: IpProtocol) -> Self {
        self.append_info_data(InfoIpTunnel::Protocol(proto))
    }

    /// This is equivalent to `mode MODE` in command
    /// `ip link add name NAME type ip6tnl mode MODE`.
    pub fn mode(self, mode: IpProtocol) -> Self {
        self.protocol(mode)
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

    /// Set arbitrary [InfoIpTunnel::Ipv6Flags]
    pub fn ipv6_flags(mut self, flags: Ip6TunnelFlags) -> Self {
        self.iface_self.flags = Some(flags);
        self
    }

    /// Set arbitrary [InfoIpTunnel::FlowInfo]
    pub fn flowinfo_raw(mut self, flowinfo: u32) -> Self {
        self.iface_self.flowinfo = Some(flowinfo);
        self
    }
}
