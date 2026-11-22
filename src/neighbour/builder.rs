// SPDX-License-Identifier: MIT

use std::net::IpAddr;

use netlink_packet_route::{
    neighbour::{
        NeighbourAddress, NeighbourAttribute, NeighbourFlags, NeighbourMessage,
        NeighbourState,
    },
    route::RouteType,
    AddressFamily,
};

/// Helper struct for building [NeighbourMessage].
#[derive(Debug, Clone)]
pub struct NeighbourMessageBuilder {
    message: NeighbourMessage,
}

impl Default for NeighbourMessageBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl NeighbourMessageBuilder {
    /// Create a new [NeighbourMessageBuilder].
    ///
    /// The default state is `NeighbourState::Permanent` and the default kind
    /// is `RouteType::Unspec`, matching `ip neighbour add`.
    pub fn new() -> Self {
        let mut message = NeighbourMessage::default();
        message.header.state = NeighbourState::Permanent;
        message.header.kind = RouteType::Unspec;
        Self { message }
    }

    /// Sets the interface index.
    pub fn index(mut self, index: u32) -> Self {
        self.message.header.ifindex = index;
        self
    }

    /// Sets the address family.
    pub fn family(mut self, family: AddressFamily) -> Self {
        self.message.header.family = family;
        self
    }

    /// Sets the destination address (`NDA_DST`).
    ///
    /// If the address family has not been set, it is derived from the
    /// destination address.
    pub fn destination(mut self, address: IpAddr) -> Self {
        if self.message.header.family == AddressFamily::Unspec {
            self.message.header.family = match address {
                IpAddr::V4(_) => AddressFamily::Inet,
                IpAddr::V6(_) => AddressFamily::Inet6,
            };
        }
        let address = match address {
            IpAddr::V4(v4) => NeighbourAddress::Inet(v4),
            IpAddr::V6(v6) => NeighbourAddress::Inet6(v6),
        };
        set_destination(&mut self.message, address);
        self
    }

    /// Sets the link layer address (`NDA_LLADDR`).
    ///
    /// On Linux, Fuchsia and Android, if the address family has not been set,
    /// it is set to `AddressFamily::Bridge`, matching `bridge fdb add`.
    pub fn link_layer_address(mut self, address: &[u8]) -> Self {
        #[cfg(any(
            target_os = "linux",
            target_os = "fuchsia",
            target_os = "android"
        ))]
        if self.message.header.family == AddressFamily::Unspec {
            self.message.header.family = AddressFamily::Bridge;
        }
        set_link_layer_address(&mut self.message, address);
        self
    }

    /// Sets a bitmask of states for the neighbour cache entry.
    pub fn state(mut self, state: NeighbourState) -> Self {
        self.message.header.state = state;
        self
    }

    /// Sets flags for the neighbour cache entry.
    pub fn flags(mut self, flags: NeighbourFlags) -> Self {
        self.message.header.flags = flags;
        self
    }

    /// Sets attributes applicable to the neighbour cache entry.
    pub fn kind(mut self, kind: RouteType) -> Self {
        self.message.header.kind = kind;
        self
    }

    /// Return a mutable reference to the message being built.
    pub fn get_mut(&mut self) -> &mut NeighbourMessage {
        &mut self.message
    }

    /// Append an arbitrary [NeighbourAttribute].
    pub fn append_attribute(mut self, attribute: NeighbourAttribute) -> Self {
        self.message.attributes.push(attribute);
        self
    }

    /// Builds [NeighbourMessage].
    pub fn build(self) -> NeighbourMessage {
        self.message
    }
}

fn set_destination(message: &mut NeighbourMessage, address: NeighbourAddress) {
    let destination = message.attributes.iter_mut().find_map(|nla| match nla {
        NeighbourAttribute::Destination(destination) => Some(destination),
        _ => None,
    });

    if let Some(destination) = destination {
        *destination = address;
    } else {
        message
            .attributes
            .push(NeighbourAttribute::Destination(address));
    }
}

fn set_link_layer_address(message: &mut NeighbourMessage, address: &[u8]) {
    let link_layer_address =
        message.attributes.iter_mut().find_map(|nla| match nla {
            NeighbourAttribute::LinkLayerAddress(lla) => Some(lla),
            _ => None,
        });

    if let Some(link_layer_address) = link_layer_address {
        *link_layer_address = address.to_vec();
    } else {
        message
            .attributes
            .push(NeighbourAttribute::LinkLayerAddress(address.to_vec()));
    }
}
