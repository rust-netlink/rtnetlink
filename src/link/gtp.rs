// SPDX-License-Identifier: MIT

use crate::{
    packet_route::link::{GtpRole, InfoData, InfoGtp, InfoKind},
    LinkMessageBuilder,
};

/// Represent GTP interface.
/// Example code on creating a GTP interface
/// ```no_run
/// use rtnetlink::{new_connection, LinkGtp};
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let (connection, handle, _) = new_connection().unwrap();
///     tokio::spawn(connection);
///
///     handle
///         .link()
///         .add(
///             LinkGtp::new("gtp0")
///                 .role(rtnetlink::packet_route::link::GtpRole::Sgsn)
///                 .build(),
///         )
///         .execute()
///         .await
///         .map_err(|e| format!("{e}"))
/// }
/// ```
///
/// Please check LinkMessageBuilder::<LinkGtp> for more detail.
#[derive(Default, Debug)]
pub struct LinkGtp;

impl LinkGtp {
    /// Equal to `LinkMessageBuilder::<LinkGtp>::new()`
    pub fn new(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkGtp>::new(name)
    }
}

impl LinkMessageBuilder<LinkGtp> {
    /// Create [LinkMessageBuilder] for GTP interface type
    pub fn new(name: &str) -> Self {
        let mut builder =
            LinkMessageBuilder::<LinkGtp>::new_with_info_kind(InfoKind::Gtp)
                .name(name.to_string());
        // iproute2 always sets create_sockets to true
        builder = builder.create_sockets(true);
        builder
    }

    fn append_info_data(self, info: InfoGtp) -> Self {
        let mut ret = self;
        if let InfoData::Gtp(infos) = ret
            .info_data
            .get_or_insert_with(|| InfoData::Gtp(Vec::new()))
        {
            infos.push(info);
        }
        ret
    }

    /// This is equivalent to `role ROLE` in command
    /// `ip link add name NAME type gtp role ROLE`.
    /// ROLE is either "sgsn" or "ggsn".
    pub fn role(self, role: GtpRole) -> Self {
        self.append_info_data(InfoGtp::Role(role))
    }

    /// This is equivalent to `hsize HSIZE` in command
    /// `ip link add name NAME type gtp hsize HSIZE`.
    pub fn pdp_hashsize(self, hsize: u32) -> Self {
        self.append_info_data(InfoGtp::PdpHashsize(hsize))
    }

    /// This is equivalent to `restart_count N` in command
    /// `ip link add name NAME type gtp restart_count N`.
    pub fn restart_count(self, count: u8) -> Self {
        self.append_info_data(InfoGtp::RestartCount(count))
    }

    /// Internal: set create_sockets flag.
    /// This is always set to true by iproute2.
    fn create_sockets(self, enabled: bool) -> Self {
        self.append_info_data(InfoGtp::CreateSockets(enabled))
    }
}
