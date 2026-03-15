// SPDX-License-Identifier: MIT

use crate::{
    link::LinkMessageBuilder,
    packet_route::link::{
        InfoData, InfoIpVtap, InfoKind, IpVtapFlags, IpVtapMode,
    },
};

/// Represent IP VTAP interface.
/// Example code on creating a IP VTAP interface
/// ```no_run
/// use rtnetlink::{new_connection, packet_route::link::{IpVtapFlags, IpVtapMode},
///                 LinkIpVtap};
///
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let (connection, handle, _) = new_connection().unwrap();
///     tokio::spawn(connection);
///
///     handle
///         .link()
///         .add(
///             LinkIpVtap::new("ipvtap100", 10, IpVtapMode::L2, IpVtapFlags::empty())
///                 .up()
///                 .build(),
///         )
///         .execute()
///         .await
///         .map_err(|e| format!("{e}"))
/// }
/// ```
///
/// Please check LinkMessageBuilder::<LinkIpVtap> for more detail.
#[derive(Debug)]
pub struct LinkIpVtap;

impl LinkIpVtap {
    /// Wrapper of `LinkMessageBuilder::<LinkIpVtap>::new().link().mode()`
    pub fn new(
        name: &str,
        base_iface_index: u32,
        mode: IpVtapMode,
        flags: IpVtapFlags,
    ) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkIpVtap>::new(name)
            .link(base_iface_index)
            .mode(mode)
            .flags(flags)
    }
}

impl LinkMessageBuilder<LinkIpVtap> {
    /// Create [LinkMessageBuilder] for IP VLAN
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

    pub fn flags(self, flags: IpVtapFlags) -> Self {
        self.append_info_data(InfoIpVtap::Flags(flags))
    }
}
