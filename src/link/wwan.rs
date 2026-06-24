// SPDX-License-Identifier: MIT

use crate::{
    packet_route::link::{InfoData, InfoKind, InfoWwan, LinkAttribute},
    LinkMessageBuilder,
};

/// Represent wwan interface.
/// Example code on creating a wwan interface.
/// The `parentdev` is required to specify the parent device for the wwan
/// interface.
/// ```no_run
/// use rtnetlink::{new_connection, LinkWwan};
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let (connection, handle, _) = new_connection().unwrap();
///     tokio::spawn(connection);
///
///     handle
///         .link()
///         .add(
///             LinkWwan::new("wwan0p")
///                 .parent_dev_name("wwan0")
///                 .linkid(15)
///                 .build(),
///         )
///         .execute()
///         .await
///         .map_err(|e| format!("{e}"))
/// }
/// ```
///
/// Please check LinkMessageBuilder::<LinkWwan> for more detail.
#[derive(Default, Debug)]
pub struct LinkWwan;

impl LinkWwan {
    /// Equal to `LinkMessageBuilder::<LinkWwan>::new()`
    pub fn new(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkWwan>::new(name)
    }
}

impl LinkMessageBuilder<LinkWwan> {
    /// Create [LinkMessageBuilder] for wwan interface type
    pub fn new(name: &str) -> Self {
        LinkMessageBuilder::<LinkWwan>::new_with_info_kind(InfoKind::Wwan)
            .name(name.to_string())
    }

    fn append_info_data(self, info: InfoWwan) -> Self {
        let mut ret = self;
        if let InfoData::Wwan(infos) = ret
            .info_data
            .get_or_insert_with(|| InfoData::Wwan(Vec::new()))
        {
            infos.push(info);
        }
        ret
    }

    /// This is equivalent to `linkid LINKID` in command
    /// `ip link add name NAME type wwan linkid LINKID`.
    pub fn linkid(self, linkid: u32) -> Self {
        self.append_info_data(InfoWwan::LinkId(linkid))
    }

    /// This is equivalent to `parentdev PARENTDEV` in command
    /// `ip link add name NAME parentdev PARENTDEV type wwan linkid LINKID`.
    /// It specifies the parent device name of the wwan interface.
    pub fn parent_dev_name(self, parent: impl Into<String>) -> Self {
        self.append_extra_attribute(LinkAttribute::ParentDevName(parent.into()))
    }
}
