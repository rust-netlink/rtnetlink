// SPDX-License-Identifier: MIT

use crate::{link::LinkMessageBuilder, packet_route::link::InfoKind};

/// Represent nlmon interface.
/// Example code on creating a nlmon interface
/// ```no_run
/// use rtnetlink::{new_connection, LinkNlmon};
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let (connection, handle, _) = new_connection().unwrap();
///     tokio::spawn(connection);
///
///     handle
///         .link()
///         .add(LinkNlmon::new("nl0").build())
///         .execute()
///         .await
///         .map_err(|e| format!("{e}"))
/// }
/// ```
#[derive(Debug)]
pub struct LinkNlmon;

impl LinkNlmon {
    /// Equal to `LinkMessageBuilder::<LinkNlmon>::new()`
    pub fn new(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkNlmon>::new(name)
    }
}

impl LinkMessageBuilder<LinkNlmon> {
    /// Create [LinkMessageBuilder] for nlmon interface type
    pub fn new(name: &str) -> Self {
        LinkMessageBuilder::<LinkNlmon>::new_with_info_kind(InfoKind::Nlmon)
            .name(name.to_string())
    }
}
