// SPDX-License-Identifier: MIT

use crate::{
    packet_route::{
        link::{InfoBareUdp, InfoData, InfoKind},
        EthernetProtocol,
    },
    LinkMessageBuilder,
};

/// Represent BareUDP interface.
#[derive(Default, Debug)]
pub struct LinkBareudp;

impl LinkBareudp {
    pub fn new(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkBareudp>::new(name)
    }
}

impl LinkMessageBuilder<LinkBareudp> {
    pub fn new(name: &str) -> Self {
        LinkMessageBuilder::<LinkBareudp>::new_with_info_kind(InfoKind::BareUdp)
            .name(name.to_string())
    }

    fn append_info_data(self, info: InfoBareUdp) -> Self {
        let mut ret = self;
        if let InfoData::BareUdp(infos) = ret
            .info_data
            .get_or_insert_with(|| InfoData::BareUdp(Vec::new()))
        {
            infos.push(info);
        }
        ret
    }

    pub fn dstport(self, port: u16) -> Self {
        self.append_info_data(InfoBareUdp::Port(port))
    }

    pub fn ethertype(self, ethertype: EthernetProtocol) -> Self {
        self.append_info_data(InfoBareUdp::Ethertype(ethertype))
    }

    pub fn srcportmin(self, port: u16) -> Self {
        self.append_info_data(InfoBareUdp::SrcPortMin(port))
    }

    pub fn multiproto(self) -> Self {
        self.append_info_data(InfoBareUdp::MultiprotoMode)
    }
}
