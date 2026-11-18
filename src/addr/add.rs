// SPDX-License-Identifier: MIT

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use futures_util::stream::StreamExt;
use netlink_packet_core::{
    NetlinkMessage, NLM_F_ACK, NLM_F_CREATE, NLM_F_EXCL, NLM_F_REPLACE,
    NLM_F_REQUEST,
};
use netlink_packet_route::{
    address::{
        AddressFlags, AddressMessage, AddressProtocol, AddressScope, CacheInfo,
    },
    RouteNetlinkMessage,
};

use crate::{
    addr::{
        builder::{
            set_anycast, set_broadcast, set_cache_info, set_flags, set_label,
            set_peer, set_preferred_lft, set_priority, set_protocol, set_scope,
            set_valid_lft,
        },
        AddressMessageBuilder,
    },
    try_nl, Error, Handle,
};

/// A request to create a new address. This is equivalent to the `ip address
/// add` commands.
pub struct AddressAddRequest {
    handle: Handle,
    message: AddressMessage,
    replace: bool,
}

impl AddressAddRequest {
    pub(crate) fn new(
        handle: Handle,
        index: u32,
        address: IpAddr,
        prefix_len: u8,
    ) -> Self {
        let message = match address {
            IpAddr::V4(address) => AddressMessageBuilder::<Ipv4Addr>::new()
                .index(index)
                .address(address, prefix_len)
                .build(),
            IpAddr::V6(address) => AddressMessageBuilder::<Ipv6Addr>::new()
                .index(index)
                .address(address, prefix_len)
                .build(),
        };

        AddressAddRequest {
            handle,
            message,
            replace: false,
        }
    }

    /// Sets the address scope (`ifa_scope`), e.g. `AddressScope::Link`.
    pub fn scope(mut self, scope: AddressScope) -> Self {
        set_scope(&mut self.message, scope);
        self
    }

    /// Sets the address flags like the `nodad`, `optimistic`, `home`,
    /// `mngtmpaddr`, `noprefixroute` and `autojoin` options of
    /// `ip address add`.
    ///
    /// The lower 8 bits are stored in the `ifa_flags` field of the message
    /// header. The full 32-bit value is sent as the `IFA_FLAGS` attribute
    /// when any bit does not fit, like iproute2 does.
    pub fn flags(mut self, flags: AddressFlags) -> Self {
        set_flags(&mut self.message, flags);
        self
    }

    /// Sets the peer address (`IFA_ADDRESS`) for a point-to-point address.
    pub fn peer(mut self, peer: IpAddr) -> Self {
        set_peer(&mut self.message, peer);
        self
    }

    /// Sets the broadcast address (`IFA_BROADCAST`).
    ///
    /// Only IPv4 addresses support broadcast.
    pub fn broadcast(mut self, broadcast: Ipv4Addr) -> Self {
        set_broadcast(&mut self.message, broadcast);
        self
    }

    /// Sets the anycast address (`IFA_ANYCAST`).
    ///
    /// Only IPv6 addresses support anycast.
    pub fn anycast(mut self, anycast: Ipv6Addr) -> Self {
        set_anycast(&mut self.message, anycast);
        self
    }

    /// Sets the interface label (`IFA_LABEL`).
    pub fn label(mut self, label: impl Into<String>) -> Self {
        set_label(&mut self.message, label.into());
        self
    }

    /// Sets the route priority (`IFA_RT_PRIORITY`), which is the `metric`
    /// option of `ip address add`.
    pub fn priority(mut self, priority: u32) -> Self {
        set_priority(&mut self.message, priority);
        self
    }

    /// Sets the cache information (`IFA_CACHEINFO`).
    pub fn cache_info(mut self, cache_info: CacheInfo) -> Self {
        set_cache_info(&mut self.message, cache_info);
        self
    }

    /// Sets the valid lifetime (`ifa_valid` of `IFA_CACHEINFO`).
    ///
    /// When no [AddressAddRequest::cache_info] is set, the preferred lifetime
    /// is set to `u32::MAX` (infinite) like iproute2 does for the omitted
    /// `preferred_lft`.
    pub fn valid_lft(mut self, valid_lft: u32) -> Self {
        set_valid_lft(&mut self.message, valid_lft);
        self
    }

    /// Sets the preferred lifetime (`ifa_preferred` of `IFA_CACHEINFO`).
    ///
    /// When no [AddressAddRequest::cache_info] is set, the valid lifetime is
    /// set to `u32::MAX` (infinite) like iproute2 does for the omitted
    /// `valid_lft`.
    pub fn preferred_lft(mut self, preferred_lft: u32) -> Self {
        set_preferred_lft(&mut self.message, preferred_lft);
        self
    }

    /// Sets the address protocol (`IFA_PROTO`).
    pub fn protocol(mut self, protocol: AddressProtocol) -> Self {
        set_protocol(&mut self.message, protocol);
        self
    }

    /// Replace existing matching address.
    pub fn replace(self) -> Self {
        Self {
            replace: true,
            ..self
        }
    }

    /// Execute the request.
    pub async fn execute(self) -> Result<(), Error> {
        let AddressAddRequest {
            mut handle,
            message,
            replace,
        } = self;
        let mut req =
            NetlinkMessage::from(RouteNetlinkMessage::NewAddress(message));
        let replace = if replace { NLM_F_REPLACE } else { NLM_F_EXCL };
        req.header.flags = NLM_F_REQUEST | NLM_F_ACK | replace | NLM_F_CREATE;

        let mut response = handle.request(req)?;
        while let Some(message) = response.next().await {
            try_nl!(message);
        }
        Ok(())
    }

    /// Return a mutable reference to the request message.
    pub fn message_mut(&mut self) -> &mut AddressMessage {
        &mut self.message
    }
}
