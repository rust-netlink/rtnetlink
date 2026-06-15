// SPDX-License-Identifier: MIT

use crate::{
    link::LinkMessageBuilder,
    packet_route::link::{
        InfoData, InfoIpVlan, InfoIpVtap, InfoKind, IpVlanFlags, IpVlanMode,
        IpVtapFlags, IpVtapMode,
    },
};

/// Represent IPVLAN interface.
#[derive(Default, Debug)]
pub struct LinkIpVlan;

impl LinkIpVlan {
    /// Wrapper of `LinkMessageBuilder::<LinkIpVlan>::new().link().mode()`
    pub fn new(
        name: &str,
        base_iface_index: u32,
        mode: IpVlanMode,
    ) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkIpVlan>::new(name)
            .link(base_iface_index)
            .mode(mode)
    }
}

impl LinkMessageBuilder<LinkIpVlan> {
    /// Create [LinkMessageBuilder] for IPVLAN interface
    pub fn new(name: &str) -> Self {
        LinkMessageBuilder::<LinkIpVlan>::new_with_info_kind(InfoKind::IpVlan)
            .name(name.to_string())
    }

    pub fn append_info_data(mut self, info: InfoIpVlan) -> Self {
        if let InfoData::IpVlan(infos) = self
            .info_data
            .get_or_insert_with(|| InfoData::IpVlan(Vec::new()))
        {
            infos.push(info);
        }
        self
    }

    pub fn mode(self, mode: IpVlanMode) -> Self {
        self.append_info_data(InfoIpVlan::Mode(mode))
    }

    pub fn flag(self, flag: IpVlanFlags) -> Self {
        self.append_info_data(InfoIpVlan::Flags(flag))
    }
}

/// Represent IPVTAP interface.
#[derive(Default, Debug)]
pub struct LinkIpVtap;

impl LinkIpVtap {
    /// Wrapper of `LinkMessageBuilder::<LinkIpVtap>::new().link().mode()`
    pub fn new(
        name: &str,
        base_iface_index: u32,
        mode: IpVtapMode,
    ) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkIpVtap>::new(name)
            .link(base_iface_index)
            .mode(mode)
    }
}

impl LinkMessageBuilder<LinkIpVtap> {
    /// Create [LinkMessageBuilder] for IPVTAP interface
    pub fn new(name: &str) -> Self {
        LinkMessageBuilder::<LinkIpVtap>::new_with_info_kind(InfoKind::IpVtap)
            .name(name.to_string())
    }

    pub fn append_info_data(mut self, info: InfoIpVtap) -> Self {
        if let InfoData::IpVtap(infos) = self
            .info_data
            .get_or_insert_with(|| InfoData::IpVtap(Vec::new()))
        {
            infos.push(info);
        }
        self
    }

    pub fn mode(self, mode: IpVtapMode) -> Self {
        self.append_info_data(InfoIpVtap::Mode(mode))
    }

    pub fn flag(self, flag: IpVtapFlags) -> Self {
        self.append_info_data(InfoIpVtap::Flags(flag))
    }
}
