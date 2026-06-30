// SPDX-License-Identifier: MIT

use std::net::{IpAddr, Ipv4Addr};

use crate::{
    packet_route::link::{InfoData, InfoKind, InfoVti},
    LinkMessageBuilder,
};

#[derive(Default, Debug)]
pub struct LinkVti;

impl LinkVti {
    pub fn new(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkVti>::new(name)
    }
}

impl LinkMessageBuilder<LinkVti> {
    pub fn new(name: &str) -> Self {
        LinkMessageBuilder::<LinkVti>::new_with_info_kind(InfoKind::Vti)
            .name(name.to_string())
    }

    fn append_info_data(self, info: InfoVti) -> Self {
        let mut ret = self;
        if let InfoData::Vti(infos) = ret
            .info_data
            .get_or_insert_with(|| InfoData::Vti(Vec::new()))
        {
            infos.push(info);
        }
        ret
    }

    pub fn local(self, addr: Ipv4Addr) -> Self {
        self.append_info_data(InfoVti::Local(IpAddr::V4(addr)))
    }

    pub fn remote(self, addr: Ipv4Addr) -> Self {
        self.append_info_data(InfoVti::Remote(IpAddr::V4(addr)))
    }

    pub fn ikey(self, key: u32) -> Self {
        self.append_info_data(InfoVti::IKey(key))
    }

    pub fn okey(self, key: u32) -> Self {
        self.append_info_data(InfoVti::OKey(key))
    }

    pub fn dev(self, ifindex: u32) -> Self {
        self.append_info_data(InfoVti::Link(ifindex))
    }

    pub fn fwmark(self, mark: u32) -> Self {
        self.append_info_data(InfoVti::FwMark(mark))
    }
}
