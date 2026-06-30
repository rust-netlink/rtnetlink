// SPDX-License-Identifier: MIT

use crate::{
    packet_route::link::{
        InfoData, InfoKind, InfoRmNet, InfoRmNetFlags, LinkAttribute,
        RmNetFlags,
    },
    LinkMessageBuilder,
};

/// Represent rmnet interface.
/// Example code on creating a rmnet interface.
/// The `link` (parent device) is required for the rmnet interface.
/// ```no_run
/// use rtnetlink::{new_connection, LinkRmNet};
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let (connection, handle, _) = new_connection().unwrap();
///     tokio::spawn(connection);
///
///     handle
///         .link()
///         .add(
///             LinkRmNet::new("rmnet0")
///                 .parent_dev_name("dummy0")
///                 .mux_id(10)
///                 .build(),
///         )
///         .execute()
///         .await
///         .map_err(|e| format!("{e}"))
/// }
/// ```
///
/// Please check LinkMessageBuilder::<LinkRmNet> for more detail.
#[derive(Default, Debug)]
pub struct LinkRmNet;

impl LinkRmNet {
    /// Equal to `LinkMessageBuilder::<LinkRmNet>::new()`
    pub fn new(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkRmNet>::new(name)
    }
}

impl LinkMessageBuilder<LinkRmNet> {
    /// Create [LinkMessageBuilder] for rmnet interface type
    pub fn new(name: &str) -> Self {
        LinkMessageBuilder::<LinkRmNet>::new_with_info_kind(InfoKind::RmNet)
            .name(name.to_string())
    }

    fn append_info_data(self, info: InfoRmNet) -> Self {
        let mut ret = self;
        if let InfoData::RmNet(infos) = ret
            .info_data
            .get_or_insert_with(|| InfoData::RmNet(Vec::new()))
        {
            infos.push(info);
        }
        ret
    }

    /// This is equivalent to `mux_id MUXID` in command
    /// `ip link add name NAME link DEVICE type rmnet mux_id MUXID`.
    pub fn mux_id(self, mux_id: u16) -> Self {
        self.append_info_data(InfoRmNet::MuxId(mux_id))
    }

    /// Set rmnet flags
    pub fn flags(self, flags: RmNetFlags, mask: RmNetFlags) -> Self {
        self.append_info_data(InfoRmNet::Flags(InfoRmNetFlags::new(
            flags, mask,
        )))
    }

    /// This is equivalent to `link DEVICE` in command
    /// `ip link add name NAME link DEVICE type rmnet mux_id MUXID`.
    /// It specifies the parent device name of the rmnet interface.
    pub fn parent_dev_name(self, parent: impl Into<String>) -> Self {
        self.append_extra_attribute(LinkAttribute::ParentDevName(parent.into()))
    }
}
