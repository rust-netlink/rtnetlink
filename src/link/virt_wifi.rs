// SPDX-License-Identifier: MIT

use crate::{link::LinkMessageBuilder, packet_route::link::InfoKind};

/// Represent virt_wifi interface.
/// Example code on creating a virt_wifi interface
/// ```no_run
/// use rtnetlink::{new_connection, LinkVirtWifi};
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let (connection, handle, _) = new_connection().unwrap();
///     tokio::spawn(connection);
///
///     handle
///         .link()
///         .add(LinkVirtWifi::new("virt_wifi0").build())
///         .execute()
///         .await
///         .map_err(|e| format!("{e}"))
/// }
/// ```
///
/// Please check LinkMessageBuilder::<LinkVirtWifi> for more detail.
#[derive(Default, Debug)]
pub struct LinkVirtWifi;

impl LinkVirtWifi {
    /// Equal to `LinkMessageBuilder::<LinkVirtWifi>::new()`
    pub fn new(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkVirtWifi>::new(name)
    }
}

impl LinkMessageBuilder<LinkVirtWifi> {
    /// Create [LinkMessageBuilder] for virt_wifi interface type
    pub fn new(name: &str) -> Self {
        LinkMessageBuilder::<LinkVirtWifi>::new_with_info_kind(
            InfoKind::VirtWifi,
        )
        .name(name.to_string())
    }
}
