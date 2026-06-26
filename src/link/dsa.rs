// SPDX-License-Identifier: MIT

use crate::{
    packet_route::link::{InfoData, InfoDsa, InfoKind},
    LinkMessageBuilder,
};

/// Represent DSA interface.
/// Example code on creating a DSA interface
/// ```no_run
/// use rtnetlink::{new_connection, LinkDsa};
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let (connection, handle, _) = new_connection().unwrap();
///     tokio::spawn(connection);
///
///     handle
///         .link()
///         .add(
///             LinkDsa::new("swp0")
///                 .conduit(3)
///                 .build(),
///         )
///         .execute()
///         .await
///         .map_err(|e| format!("{e}"))
/// }
/// ```
///
/// Please check LinkMessageBuilder::<LinkDsa> for more detail.
#[derive(Default, Debug)]
pub struct LinkDsa;

impl LinkDsa {
    /// Equal to `LinkMessageBuilder::<LinkDsa>::new()`
    pub fn new(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkDsa>::new(name)
    }
}

impl LinkMessageBuilder<LinkDsa> {
    /// Create [LinkMessageBuilder] for DSA interface type
    pub fn new(name: &str) -> Self {
        LinkMessageBuilder::<LinkDsa>::new_with_info_kind(InfoKind::Dsa)
            .name(name.to_string())
    }

    fn append_info_data(self, info: InfoDsa) -> Self {
        let mut ret = self;
        if let InfoData::Dsa(infos) = ret
            .info_data
            .get_or_insert_with(|| InfoData::Dsa(Vec::new()))
        {
            infos.push(info);
        }
        ret
    }

    /// This is equivalent to `conduit IFINDEX` in command
    /// `ip link set dev DEV type dsa conduit DEVICE`.
    pub fn conduit(self, ifindex: u32) -> Self {
        self.append_info_data(InfoDsa::Conduit(ifindex))
    }
}
