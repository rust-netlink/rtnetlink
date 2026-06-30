// SPDX-License-Identifier: MIT

use std::net::Ipv6Addr;

use crate::{
    packet_route::link::{
        ErSpanDir, GreEncapFlags, GreEncapType, GreIOFlags, InfoData, InfoGre6,
        InfoKind,
    },
    LinkMessageBuilder,
};

/// Represent IP6GRE/IP6Gretap interface (IPv6).
#[derive(Default, Debug)]
pub struct LinkGre6;

impl LinkGre6 {
    pub fn new(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkGre6>::new_gre6(name)
    }

    pub fn new_gretap6(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkGre6>::new_gretap6(name)
    }

    pub fn new_ip6erspan(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkGre6>::new_ip6erspan(name)
    }
}

impl LinkMessageBuilder<LinkGre6> {
    pub fn new_gre6(name: &str) -> Self {
        LinkMessageBuilder::<LinkGre6>::new_with_info_kind(InfoKind::GreTun6)
            .name(name.to_string())
    }

    pub fn new_gretap6(name: &str) -> Self {
        LinkMessageBuilder::<LinkGre6>::new_with_info_kind(InfoKind::GreTap6)
            .name(name.to_string())
    }

    pub fn new_ip6erspan(name: &str) -> Self {
        LinkMessageBuilder::<LinkGre6>::new_with_info_kind(InfoKind::Ip6ErSpan)
            .name(name.to_string())
    }

    fn append_info_data(self, info: InfoGre6) -> Self {
        let mut ret = self;
        let kind = ret.info_kind.clone();
        if let InfoData::GreTun6(infos)
        | InfoData::GreTap6(infos)
        | InfoData::Ip6ErSpan(infos) =
            ret.info_data.get_or_insert_with(|| match kind {
                Some(InfoKind::GreTap6) => InfoData::GreTap6(Vec::new()),
                Some(InfoKind::Ip6ErSpan) => InfoData::Ip6ErSpan(Vec::new()),
                _ => InfoData::GreTun6(Vec::new()),
            })
        {
            infos.push(info);
        }
        ret
    }

    pub fn local(self, addr: Ipv6Addr) -> Self {
        self.append_info_data(InfoGre6::Local(addr))
    }

    pub fn remote(self, addr: Ipv6Addr) -> Self {
        self.append_info_data(InfoGre6::Remote(addr))
    }

    pub fn ttl(self, ttl: u8) -> Self {
        self.append_info_data(InfoGre6::Ttl(ttl))
    }

    pub fn collect_metadata(self, enabled: bool) -> Self {
        if enabled {
            self.append_info_data(InfoGre6::CollectMetadata)
        } else {
            self
        }
    }

    pub fn fwmark(self, mark: u32) -> Self {
        self.append_info_data(InfoGre6::FwMask(mark))
    }

    pub fn erspan_index(self, index: u32) -> Self {
        self.append_info_data(InfoGre6::ErSpanIndex(index))
    }

    pub fn erspan_ver(self, ver: u8) -> Self {
        self.append_info_data(InfoGre6::ErSpanVer(ver))
    }

    pub fn erspan_dir(self, dir: ErSpanDir) -> Self {
        self.append_info_data(InfoGre6::ErSpanDir(dir))
    }

    pub fn erspan_hwid(self, hwid: u16) -> Self {
        self.append_info_data(InfoGre6::ErSpanHwId(hwid))
    }

    pub fn iflags(self, flags: GreIOFlags) -> Self {
        self.append_info_data(InfoGre6::IFlags(flags))
    }

    pub fn oflags(self, flags: GreIOFlags) -> Self {
        self.append_info_data(InfoGre6::OFlags(flags))
    }

    pub fn ikey(self, key: u32) -> Self {
        self.append_info_data(InfoGre6::IKey(key))
    }

    pub fn okey(self, key: u32) -> Self {
        self.append_info_data(InfoGre6::OKey(key))
    }

    pub fn encap_limit(self, limit: u8) -> Self {
        self.append_info_data(InfoGre6::EncapLimit(limit))
    }

    pub fn flowlabel(self, label: u32) -> Self {
        self.append_info_data(InfoGre6::FlowLabel(label))
    }

    pub fn encap_type(self, encap_type: GreEncapType) -> Self {
        self.append_info_data(InfoGre6::EncapType(encap_type))
    }

    pub fn encap_sport(self, port: u16) -> Self {
        self.append_info_data(InfoGre6::SourcePort(port))
    }

    pub fn encap_dport(self, port: u16) -> Self {
        self.append_info_data(InfoGre6::DestinationPort(port))
    }

    pub fn encap_flags(self, flags: GreEncapFlags) -> Self {
        self.append_info_data(InfoGre6::EncapFlags(flags))
    }
}
