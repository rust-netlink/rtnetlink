// SPDX-License-Identifier: MIT

use crate::{link::LinkMessageBuilder, packet_route::link::InfoKind};

/// Represent vcan interface.
/// Example code on creating a vcan interface
/// ```no_run
/// use rtnetlink::{new_connection, LinkVcan};
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let (connection, handle, _) = new_connection().unwrap();
///     tokio::spawn(connection);
///
///     handle
///         .link()
///         .add(LinkVcan::new("vcan0").build())
///         .execute()
///         .await
///         .map_err(|e| format!("{e}"))
/// }
/// ```
#[derive(Default, Debug)]
pub struct LinkVcan;

impl LinkVcan {
    /// Equal to `LinkMessageBuilder::<LinkVcan>::new()`
    pub fn new(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkVcan>::new(name)
    }
}

impl LinkMessageBuilder<LinkVcan> {
    /// Create [LinkMessageBuilder] for vcan interface type
    pub fn new(name: &str) -> Self {
        LinkMessageBuilder::<LinkVcan>::new_with_info_kind(InfoKind::Vcan)
            .name(name.to_string())
    }
}
