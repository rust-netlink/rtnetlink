// SPDX-License-Identifier: MIT

use std::net::Ipv4Addr;

use crate::{
    packet_route::link::{
        GreEncapFlags, GreEncapType, GreIOFlags, InfoData, InfoGre, InfoKind,
    },
    LinkMessageBuilder,
};

/// Represent GRE/Gretap interface (IPv4).
#[derive(Default, Debug)]
pub struct LinkGre;

impl LinkGre {
    pub fn new(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkGre>::new_gre(name)
    }

    pub fn new_gretap(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkGre>::new_gretap(name)
    }
}

impl LinkMessageBuilder<LinkGre> {
    pub fn new_gre(name: &str) -> Self {
        LinkMessageBuilder::<LinkGre>::new_with_info_kind(InfoKind::GreTun)
            .name(name.to_string())
    }

    pub fn new_gretap(name: &str) -> Self {
        LinkMessageBuilder::<LinkGre>::new_with_info_kind(InfoKind::GreTap)
            .name(name.to_string())
    }

    fn append_info_data(self, info: InfoGre) -> Self {
        let mut ret = self;
        let kind = ret.info_kind.clone();
        if let InfoData::GreTun(infos) | InfoData::GreTap(infos) =
            ret.info_data.get_or_insert_with(|| match kind {
                Some(InfoKind::GreTap) => InfoData::GreTap(Vec::new()),
                _ => InfoData::GreTun(Vec::new()),
            })
        {
            infos.push(info);
        }
        ret
    }

    pub fn local(self, addr: Ipv4Addr) -> Self {
        self.append_info_data(InfoGre::Local(addr))
    }

    pub fn remote(self, addr: Ipv4Addr) -> Self {
        self.append_info_data(InfoGre::Remote(addr))
    }

    pub fn ttl(self, ttl: u8) -> Self {
        self.append_info_data(InfoGre::Ttl(ttl))
    }

    pub fn tos(self, tos: u8) -> Self {
        self.append_info_data(InfoGre::Tos(tos))
    }

    pub fn pmtudisc(self, enabled: bool) -> Self {
        self.append_info_data(InfoGre::PathMTUDiscovery(enabled))
    }

    pub fn dev(self, ifindex: u32) -> Self {
        self.append_info_data(InfoGre::Link(ifindex))
    }

    pub fn collect_metadata(self, enabled: bool) -> Self {
        if enabled {
            self.append_info_data(InfoGre::CollectMetadata)
        } else {
            self
        }
    }

    pub fn fwmark(self, mark: u32) -> Self {
        self.append_info_data(InfoGre::FwMask(mark))
    }

    pub fn iflags(self, flags: GreIOFlags) -> Self {
        self.append_info_data(InfoGre::IFlags(flags))
    }

    pub fn oflags(self, flags: GreIOFlags) -> Self {
        self.append_info_data(InfoGre::OFlags(flags))
    }

    pub fn ikey(self, key: u32) -> Self {
        self.append_info_data(InfoGre::IKey(key))
    }

    pub fn okey(self, key: u32) -> Self {
        self.append_info_data(InfoGre::OKey(key))
    }

    pub fn encap_type(self, encap_type: GreEncapType) -> Self {
        self.append_info_data(InfoGre::EncapType(encap_type))
    }

    pub fn encap_sport(self, port: u16) -> Self {
        self.append_info_data(InfoGre::SourcePort(port))
    }

    pub fn encap_dport(self, port: u16) -> Self {
        self.append_info_data(InfoGre::DestinationPort(port))
    }

    pub fn encap_flags(self, flags: GreEncapFlags) -> Self {
        self.append_info_data(InfoGre::EncapFlags(flags))
    }
}
