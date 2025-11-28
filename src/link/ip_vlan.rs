// SPDX-License-Identifier: MIT

use crate::{
    link::LinkMessageBuilder,
    packet_route::link::{
        InfoData, InfoIpVlan, InfoKind, IpVlanFlags, IpVlanMode,
    },
};

/// Represent IP VLAN interface.
/// Example code on creating a IP VLAN interface
/// ```no_run
/// use rtnetlink::{new_connection, packet_route::link::{IpVlanFlags, IpVlanMode},
///                 LinkIpVlan};
///
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let (connection, handle, _) = new_connection().unwrap();
///     tokio::spawn(connection);
///
///     handle
///         .link()
///         .add(
///             LinkIpVlan::new("ipvlan100", 10, IpVlanMode::L2, IpVlanFlags::empty())
///                 .up()
///                 .build(),
///         )
///         .execute()
///         .await
///         .map_err(|e| format!("{e}"))
/// }
/// ```
///
/// Please check LinkMessageBuilder::<LinkIpVlan> for more detail.
#[derive(Debug)]
pub struct LinkIpVlan;

impl LinkIpVlan {
    /// Wrapper of `LinkMessageBuilder::<LinkIpVlan>::new().link().mode()`
    pub fn new(
        name: &str,
        base_iface_index: u32,
        mode: IpVlanMode,
        flags: IpVlanFlags,
    ) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkIpVlan>::new(name)
            .link(base_iface_index)
            .mode(mode)
            .flags(flags)
    }
}

impl LinkMessageBuilder<LinkIpVlan> {
    /// Create [LinkMessageBuilder] for IP VLAN
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

    pub fn flags(self, flags: IpVlanFlags) -> Self {
        self.append_info_data(InfoIpVlan::Flags(flags))
    }
}
