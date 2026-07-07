// SPDX-License-Identifier: MIT

use crate::{link::LinkMessageBuilder, packet_route::link::InfoKind};

/// Represent TUN/TAP interface.
/// Example code on creating a TUN interface
/// ```no_run
/// use rtnetlink::{new_connection, LinkTun};
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let (connection, handle, _) = new_connection().unwrap();
///     tokio::spawn(connection);
///
///     handle
///         .link()
///         .add(LinkTun::new("tun0").build())
///         .execute()
///         .await
///         .map_err(|e| format!("{e}"))
/// }
/// ```
///
/// Please check LinkMessageBuilder::<LinkTun> for more detail.
#[derive(Default, Debug)]
pub struct LinkTun;

impl LinkTun {
    /// Equal to `LinkMessageBuilder::<LinkTun>::new()`
    pub fn new(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkTun>::new(name)
    }
}

impl LinkMessageBuilder<LinkTun> {
    /// Create [LinkMessageBuilder] for TUN interface type
    pub fn new(name: &str) -> Self {
        LinkMessageBuilder::<LinkTun>::new_with_info_kind(InfoKind::Tun)
            .name(name.to_string())
    }
}
