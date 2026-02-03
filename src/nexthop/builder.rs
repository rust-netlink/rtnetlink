// SPDX-License-Identifier: MIT

use netlink_packet_route::{
    nexthop::{
        NexthopAttribute, NexthopFlags, NexthopGroupEntry, NexthopMessage,
    },
    route::{RouteProtocol, RouteScope},
    AddressFamily,
};
use std::{
    marker::PhantomData,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};

/// A builder for [`NexthopMessage`]
#[derive(Debug)]
pub struct NexthopMessageBuilder<T = IpAddr> {
    message: NexthopMessage,
    _phantom: PhantomData<T>,
}

impl<T> NexthopMessageBuilder<T> {
    /// Create a new builder without specifying address family
    fn new_no_address_family() -> Self {
        let mut message = NexthopMessage::default();
        message.header.protocol = u8::from(RouteProtocol::Static);
        message.header.scope = u8::from(RouteScope::Universe);
        Self {
            message,
            _phantom: PhantomData,
        }
    }

    /// Set the nexthop ID
    pub fn id(mut self, id: u32) -> Self {
        self.message.nlas.push(NexthopAttribute::Id(id));
        self
    }

    /// Set the interface index
    pub fn oif(mut self, index: u32) -> Self {
        self.message.nlas.push(NexthopAttribute::Oif(index));
        self
    }

    /// Set the nexthop as blackhole
    pub fn blackhole(mut self) -> Self {
        self.message.nlas.push(NexthopAttribute::Blackhole);
        self
    }

    /// Set the nexthop group
    pub fn group(mut self, entries: Vec<(u32, u8)>) -> Self {
        let group_entries = entries
            .into_iter()
            .map(|(id, weight)| NexthopGroupEntry {
                id,
                weight,
                resvd1: 0,
                resvd2: 0,
            })
            .collect();
        self.message
            .nlas
            .push(NexthopAttribute::Group(group_entries));
        self
    }

    /// Set flags
    pub fn flags(mut self, flags: NexthopFlags) -> Self {
        self.message.header.flags = flags;
        self
    }

    /// Set the nexthop protocol
    pub fn protocol(mut self, protocol: RouteProtocol) -> Self {
        self.message.header.protocol = u8::from(protocol);
        self
    }

    /// Set the nexthop scope
    pub fn scope(mut self, scope: RouteScope) -> Self {
        self.message.header.scope = u8::from(scope);
        self
    }

    /// Build the message
    pub fn build(self) -> NexthopMessage {
        self.message
    }
}

impl Default for NexthopMessageBuilder<Ipv4Addr> {
    fn default() -> Self {
        Self::new()
    }
}

impl NexthopMessageBuilder<Ipv4Addr> {
    /// Create a new builder for IPv4
    pub fn new() -> Self {
        let mut builder = Self::new_no_address_family();
        builder.message.header.family = AddressFamily::Inet;
        builder
    }

    /// Set the gateway IP address
    pub fn gateway(mut self, addr: Ipv4Addr) -> Self {
        self.message
            .nlas
            .push(NexthopAttribute::Gateway(addr.octets().to_vec()));
        self
    }
}

impl Default for NexthopMessageBuilder<Ipv6Addr> {
    fn default() -> Self {
        Self::new()
    }
}

impl NexthopMessageBuilder<Ipv6Addr> {
    /// Create a new builder for IPv6
    pub fn new() -> Self {
        let mut builder = Self::new_no_address_family();
        builder.message.header.family = AddressFamily::Inet6;
        builder
    }

    /// Set the gateway IP address
    pub fn gateway(mut self, addr: Ipv6Addr) -> Self {
        self.message
            .nlas
            .push(NexthopAttribute::Gateway(addr.octets().to_vec()));
        self
    }
}

impl Default for NexthopMessageBuilder<IpAddr> {
    fn default() -> Self {
        Self::new()
    }
}

impl NexthopMessageBuilder<IpAddr> {
    /// Create a new builder for any IP address family
    pub fn new() -> Self {
        Self::new_no_address_family()
    }

    /// Set the gateway IP address
    pub fn gateway(mut self, addr: IpAddr) -> Self {
        let (family, bytes) = match addr {
            IpAddr::V4(addr) => (AddressFamily::Inet, addr.octets().to_vec()),
            IpAddr::V6(addr) => (AddressFamily::Inet6, addr.octets().to_vec()),
        };
        // Only set family if not already set or if explicitly different (though usually we trust the caller)
        if self.message.header.family == AddressFamily::Unspec {
            self.message.header.family = family;
        }
        self.message.nlas.push(NexthopAttribute::Gateway(bytes));
        self
    }
}
