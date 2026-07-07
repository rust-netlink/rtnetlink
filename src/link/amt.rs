// SPDX-License-Identifier: MIT

use std::net::IpAddr;

use crate::{
    packet_route::link::{AmtMode, InfoAmt, InfoData, InfoKind},
    LinkMessageBuilder,
};

/// Represent AMT (Automatic Multicast Tunneling) interface.
/// Example code on creating an AMT interface
/// ```no_run
/// use std::net::{IpAddr, Ipv4Addr};
/// use rtnetlink::{new_connection, LinkAmt};
/// use rtnetlink::packet_route::link::AmtMode;
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let (connection, handle, _) = new_connection().unwrap();
///     tokio::spawn(connection);
///
///     handle
///         .link()
///         .add(
///             LinkAmt::new("amt0")
///                 .mode(AmtMode::Gateway)
///                 .dev(2)
///                 .local_ip(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)))
///                 .build(),
///         )
///         .execute()
///         .await
///         .map_err(|e| format!("{e}"))
/// }
/// ```
///
/// Please check LinkMessageBuilder::<LinkAmt> for more detail.
#[derive(Default, Debug)]
pub struct LinkAmt;

impl LinkAmt {
    /// Equal to `LinkMessageBuilder::<LinkAmt>::new()`
    pub fn new(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkAmt>::new(name)
    }
}

impl LinkMessageBuilder<LinkAmt> {
    /// Create [LinkMessageBuilder] for AMT interface type
    pub fn new(name: &str) -> Self {
        LinkMessageBuilder::<LinkAmt>::new_with_info_kind(InfoKind::Amt)
            .name(name.to_string())
    }

    fn append_info_data(self, info: InfoAmt) -> Self {
        let mut ret = self;
        if let InfoData::Amt(infos) = ret
            .info_data
            .get_or_insert_with(|| InfoData::Amt(Vec::new()))
        {
            infos.push(info);
        }
        ret
    }

    pub fn mode(self, mode: AmtMode) -> Self {
        self.append_info_data(InfoAmt::Mode(mode))
    }

    pub fn relay_port(self, port: u16) -> Self {
        self.append_info_data(InfoAmt::RelayPort(port))
    }

    pub fn gateway_port(self, port: u16) -> Self {
        self.append_info_data(InfoAmt::GatewayPort(port))
    }

    /// Set the AMT link device (IFLA_AMT_LINK inside IFLA_INFO_DATA).
    /// This is the `dev` parameter in `ip link add ... type amt dev DEV`.
    pub fn dev(self, ifindex: u32) -> Self {
        self.append_info_data(InfoAmt::Link(ifindex))
    }

    pub fn local_ip(self, ip: IpAddr) -> Self {
        self.append_info_data(InfoAmt::LocalIp(ip))
    }

    pub fn remote_ip(self, ip: IpAddr) -> Self {
        self.append_info_data(InfoAmt::RemoteIp(ip))
    }

    pub fn discovery_ip(self, ip: IpAddr) -> Self {
        self.append_info_data(InfoAmt::DiscoveryIp(ip))
    }

    pub fn max_tunnels(self, count: u32) -> Self {
        self.append_info_data(InfoAmt::MaxTunnels(count))
    }
}
