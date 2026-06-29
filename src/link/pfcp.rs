// SPDX-License-Identifier: MIT

use crate::{link::LinkMessageBuilder, packet_route::link::InfoKind};

/// Represent PFCP interface.
/// Example code on creating a PFCP interface
/// ```no_run
/// use rtnetlink::{new_connection, LinkPfcp};
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let (connection, handle, _) = new_connection().unwrap();
///     tokio::spawn(connection);
///
///     handle
///         .link()
///         .add(LinkPfcp::new("pfcp0").build())
///         .execute()
///         .await
///         .map_err(|e| format!("{e}"))
/// }
/// ```
#[derive(Default, Debug)]
pub struct LinkPfcp;

impl LinkPfcp {
    /// Equal to `LinkMessageBuilder::<LinkPfcp>::new()`
    pub fn new(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkPfcp>::new(name)
    }
}

impl LinkMessageBuilder<LinkPfcp> {
    /// Create [LinkMessageBuilder] for PFCP interface type
    pub fn new(name: &str) -> Self {
        LinkMessageBuilder::<LinkPfcp>::new_with_info_kind(InfoKind::Pfcp)
            .name(name.to_string())
    }
}
