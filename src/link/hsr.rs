// SPDX-License-Identifier: MIT

use crate::{
    packet_route::link::{HsrProtocol, InfoData, InfoHsr, InfoKind},
    LinkMessageBuilder,
};

/// Represent HSR (High-availability Seamless Redundancy) interface.
///
/// Example code on creating a HSR interface
/// ```no_run
/// use rtnetlink::{new_connection, LinkHsr};
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let (connection, handle, _) = new_connection().unwrap();
///     tokio::spawn(connection);
///
///     handle
///         .link()
///         .add(
///             LinkHsr::new("hsr0")
///                 .port1(3)
///                 .port2(4)
///                 .build()
///         )
///         .execute()
///         .await
///         .map_err(|e| format!("{e}"))
/// }
/// ```
///
/// Please check LinkMessageBuilder::<LinkHsr> for more detail.
#[derive(Debug)]
pub struct LinkHsr;

impl LinkHsr {
    /// Equal to `LinkMessageBuilder::<LinkHsr>::new(name)`
    pub fn new(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkHsr>::new(name)
    }
}

impl LinkMessageBuilder<LinkHsr> {
    /// Create [LinkMessageBuilder] for HSR
    pub fn new(name: &str) -> Self {
        LinkMessageBuilder::<LinkHsr>::new_with_info_kind(InfoKind::Hsr)
            .name(name.to_string())
    }

    pub fn append_info_data(mut self, info: InfoHsr) -> Self {
        if let InfoData::Hsr(infos) = self
            .info_data
            .get_or_insert_with(|| InfoData::Hsr(Vec::new()))
        {
            infos.push(info);
        }
        self
    }

    /// First port device ifindex
    pub fn port1(self, ifindex: u32) -> Self {
        self.append_info_data(InfoHsr::Port1(ifindex))
    }

    /// Second port device ifindex
    pub fn port2(self, ifindex: u32) -> Self {
        self.append_info_data(InfoHsr::Port2(ifindex))
    }

    /// Interlink device ifindex for SAN connectivity
    pub fn interlink(self, ifindex: u32) -> Self {
        self.append_info_data(InfoHsr::Interlink(ifindex))
    }

    /// Last byte of the multicast address used for HSR supervision frames
    /// (0-255, default = 0)
    pub fn supervision(self, byte: u8) -> Self {
        self.append_info_data(InfoHsr::MulticastSpec(byte))
    }

    /// HSR protocol version
    pub fn version(self, version: u8) -> Self {
        self.append_info_data(InfoHsr::Version(version))
    }

    /// Protocol
    pub fn protocol(self, protocol: HsrProtocol) -> Self {
        self.append_info_data(InfoHsr::Protocol(protocol))
    }
}
