// SPDX-License-Identifier: MIT

use crate::{
    packet_route::link::{InfoData, InfoKind, InfoVxcan},
    LinkMessageBuilder, LinkUnspec,
};

/// Represent virtual can interface.
/// Example code on creating a vxcan pair
/// ```no_run
/// use rtnetlink::{new_connection, LinkVxcan};
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let (connection, handle, _) = new_connection().unwrap();
///     tokio::spawn(connection);
///
///     handle
///         .link()
///         .add(LinkVxcan::new("vxcan0", "vxcan1").build())
///         .execute()
///         .await
///         .map_err(|e| format!("{e}"))
/// }
/// ```
///
/// Please check LinkMessageBuilder::<LinkVxcan> for more detail.
#[derive(Debug)]
pub struct LinkVxcan;

impl LinkVxcan {
    /// Equal to `LinkMessageBuilder::<LinkVxcan>::new(name, peer)`
    pub fn new(name: &str, peer: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkVxcan>::new(name, peer)
    }
}

impl LinkMessageBuilder<LinkVxcan> {
    /// Create [LinkMessageBuilder] for Vxcan
    pub fn new(name: &str, peer: &str) -> Self {
        LinkMessageBuilder::<LinkVxcan>::new_with_info_kind(InfoKind::Vxcan)
            .name(name.to_string())
            .peer(peer)
    }

    pub fn peer(mut self, peer: &str) -> Self {
        let peer_msg = LinkMessageBuilder::<LinkUnspec>::new()
            .name(peer.to_string())
            .build();

        self.info_data = Some(InfoData::Vxcan(InfoVxcan::Peer(peer_msg)));
        self
    }
}
