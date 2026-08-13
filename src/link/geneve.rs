// SPDX-License-Identifier: MIT

use std::net::{Ipv4Addr, Ipv6Addr};

use crate::{
    packet_route::link::{GeneveDf, InfoData, InfoGeneve, InfoKind},
    LinkMessageBuilder,
};

/// Represent GENEVE interface.
/// Example code on creating a GENEVE interface
/// ```no_run
/// use rtnetlink::{new_connection, LinkGeneve};
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let (connection, handle, _) = new_connection().unwrap();
///     tokio::spawn(connection);
///
///     handle
///         .link()
///         .add(LinkGeneve::new("geneve0", 10)
///         .remote("10.0.0.1".parse().unwrap())
///         .up()
///         .build())
///         .execute()
///         .await
///         .map_err(|e| format!("{e}"))
/// }
/// ```
///
/// Please check LinkMessageBuilder::<LinkGeneve> for more detail.
#[derive(Default, Debug)]
pub struct LinkGeneve;

impl LinkGeneve {
    /// Wrapper of `LinkMessageBuilder::<LinkGeneve>::new().id()`
    pub fn new(name: &str, vni: u32) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkGeneve>::new(name).id(vni)
    }
}

impl LinkMessageBuilder<LinkGeneve> {
    /// Create [LinkMessageBuilder] for GENEVE
    pub fn new(name: &str) -> Self {
        LinkMessageBuilder::<LinkGeneve>::new_with_info_kind(InfoKind::Geneve)
            .name(name.to_string())
    }

    pub fn append_info_data(self, info: InfoGeneve) -> Self {
        let mut ret = self;
        if let InfoData::Geneve(infos) = ret
            .info_data
            .get_or_insert_with(|| InfoData::Geneve(Vec::new()))
        {
            infos.push(info);
        }
        ret
    }

    /// VNI
    pub fn id(self, id: u32) -> Self {
        self.append_info_data(InfoGeneve::Id(id))
    }

    /// Remote IPv4 address
    pub fn remote(self, addr: Ipv4Addr) -> Self {
        self.append_info_data(InfoGeneve::Remote(addr))
    }

    /// Remote IPv6 address
    pub fn remote6(self, addr: Ipv6Addr) -> Self {
        self.append_info_data(InfoGeneve::Remote6(addr))
    }

    /// Local IPv4 address of the backend UDP socket
    pub fn local(self, addr: Ipv4Addr) -> Self {
        self.append_info_data(InfoGeneve::Local(addr))
    }

    /// Local IPv6 address of the backend UDP socket
    pub fn local6(self, addr: Ipv6Addr) -> Self {
        self.append_info_data(InfoGeneve::Local6(addr))
    }

    pub fn ttl(self, ttl: u8) -> Self {
        self.append_info_data(InfoGeneve::Ttl(ttl))
    }

    pub fn tos(self, tos: u8) -> Self {
        self.append_info_data(InfoGeneve::Tos(tos))
    }

    /// UDP destination port
    pub fn port(self, port: u16) -> Self {
        self.append_info_data(InfoGeneve::Port(port))
    }

    pub fn collect_metadata(self) -> Self {
        self.append_info_data(InfoGeneve::CollectMetadata)
    }

    pub fn udp_csum(self, enable: bool) -> Self {
        self.append_info_data(InfoGeneve::UdpCsum(enable))
    }

    pub fn udp_zero_csum6_tx(self, enable: bool) -> Self {
        self.append_info_data(InfoGeneve::UdpZeroCsum6Tx(enable))
    }

    pub fn udp_zero_csum6_rx(self, enable: bool) -> Self {
        self.append_info_data(InfoGeneve::UdpZeroCsum6Rx(enable))
    }

    pub fn label(self, label: u32) -> Self {
        self.append_info_data(InfoGeneve::Label(label))
    }

    pub fn ttl_inherit(self, enable: bool) -> Self {
        self.append_info_data(InfoGeneve::TtlInherit(enable))
    }

    pub fn df(self, df: GeneveDf) -> Self {
        self.append_info_data(InfoGeneve::Df(df))
    }

    pub fn inner_proto_inherit(self) -> Self {
        self.append_info_data(InfoGeneve::InnerProtoInherit)
    }
}
