// SPDX-License-Identifier: MIT

use crate::{
    packet_route::link::{InfoBatAdv, InfoData, InfoKind},
    LinkMessageBuilder,
};

/// Represent batadv interface.
#[derive(Default, Debug)]
pub struct LinkBatAdv;

impl LinkBatAdv {
    pub fn new(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkBatAdv>::new(name)
    }
}

impl LinkMessageBuilder<LinkBatAdv> {
    pub fn new(name: &str) -> Self {
        LinkMessageBuilder::<LinkBatAdv>::new_with_info_kind(InfoKind::BatAdv)
            .name(name.to_string())
    }

    fn append_info_data(self, info: InfoBatAdv) -> Self {
        let mut ret = self;
        if let InfoData::BatAdv(infos) = ret
            .info_data
            .get_or_insert_with(|| InfoData::BatAdv(Vec::new()))
        {
            infos.push(info);
        }
        ret
    }

    /// Set routing algorithm (`ra` in ip command).
    pub fn algo_name(self, name: String) -> Self {
        self.append_info_data(InfoBatAdv::AlgoName(name))
    }
}
