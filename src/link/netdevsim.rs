// SPDX-License-Identifier: MIT

use crate::{link::LinkMessageBuilder, packet_route::link::InfoKind};

/// Represent netdevsim interface.
/// Example code on creating a netdevsim interface
/// ```no_run
/// use rtnetlink::{new_connection, LinkNetdevsim};
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let (connection, handle, _) = new_connection().unwrap();
///     tokio::spawn(connection);
///
///     handle
///         .link()
///         .add(LinkNetdevsim::new("netdevsim0").build())
///         .execute()
///         .await
///         .map_err(|e| format!("{e}"))
/// }
/// ```
#[derive(Default, Debug)]
pub struct LinkNetdevsim;

impl LinkNetdevsim {
    /// Equal to `LinkMessageBuilder::<LinkNetdevsim>::new()`
    pub fn new(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkNetdevsim>::new(name)
    }
}

impl LinkMessageBuilder<LinkNetdevsim> {
    /// Create [LinkMessageBuilder] for netdevsim interface type
    pub fn new(name: &str) -> Self {
        LinkMessageBuilder::<LinkNetdevsim>::new_with_info_kind(InfoKind::Netdevsim)
            .name(name.to_string())
    }
}
