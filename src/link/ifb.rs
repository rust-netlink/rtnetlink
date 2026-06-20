// SPDX-License-Identifier: MIT

use crate::{link::LinkMessageBuilder, packet_route::link::InfoKind};

/// Represent ifb interface.
/// Example code on creating a ifb interface
/// ```no_run
/// use rtnetlink::{new_connection, LinkIfb};
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let (connection, handle, _) = new_connection().unwrap();
///     tokio::spawn(connection);
///
///     handle
///         .link()
///         .add(LinkIfb::new("ifb0").build())
///         .execute()
///         .await
///         .map_err(|e| format!("{e}"))
/// }
/// ```
///
/// Please check LinkMessageBuilder::<LinkIfb> for more detail.
#[derive(Default, Debug)]
pub struct LinkIfb;

impl LinkIfb {
    /// Equal to `LinkMessageBuilder::<LinkIfb>::new()`
    pub fn new(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkIfb>::new(name)
    }
}

impl LinkMessageBuilder<LinkIfb> {
    /// Create [LinkMessageBuilder] for ifb interface type
    pub fn new(name: &str) -> Self {
        LinkMessageBuilder::<LinkIfb>::new_with_info_kind(InfoKind::Ifb)
            .name(name.to_string())
    }
}
