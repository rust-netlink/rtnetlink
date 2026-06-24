// SPDX-License-Identifier: MIT

use crate::{link::LinkMessageBuilder, packet_route::link::InfoKind};

/// Represent team interface.
/// Example code on creating a team interface
/// ```no_run
/// use rtnetlink::{new_connection, LinkTeam};
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let (connection, handle, _) = new_connection().unwrap();
///     tokio::spawn(connection);
///
///     handle
///         .link()
///         .add(LinkTeam::new("team0").build())
///         .execute()
///         .await
///         .map_err(|e| format!("{e}"))
/// }
/// ```
///
/// Please check LinkMessageBuilder::<LinkTeam> for more detail.
#[derive(Default, Debug)]
pub struct LinkTeam;

impl LinkTeam {
    /// Equal to `LinkMessageBuilder::<LinkTeam>::new()`
    pub fn new(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkTeam>::new(name)
    }
}

impl LinkMessageBuilder<LinkTeam> {
    /// Create [LinkMessageBuilder] for team interface type
    pub fn new(name: &str) -> Self {
        LinkMessageBuilder::<LinkTeam>::new_with_info_kind(InfoKind::Team)
            .name(name.to_string())
    }
}
