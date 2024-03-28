// SPDX-License-Identifier: MIT

use futures::stream::StreamExt;
use std::{
    marker::PhantomData,
    net::{Ipv4Addr, Ipv6Addr},
};

use netlink_packet_core::{
    NetlinkMessage, NLM_F_ACK, NLM_F_APPEND, NLM_F_CREATE, NLM_F_EXCL,
    NLM_F_REPLACE, NLM_F_REQUEST,
};
use netlink_packet_route::{
    route::{
        RouteAddress, RouteAttribute, RouteHeader, RouteMessage, RouteProtocol,
        RouteScope, RouteType,
    },
    AddressFamily, RouteNetlinkMessage,
};

use crate::{try_nl, Error, Handle};

/// A request to create a new route. This is equivalent to the `ip route add`
/// commands.
pub struct RouteAddRequest<T = ()> {
    handle: Handle,
    message: RouteMessage,
    nl_msg_flags: u16,
    _phantom: PhantomData<T>,
}

impl<T> RouteAddRequest<T> {
    pub(crate) fn new(handle: Handle) -> Self {
        let mut message = RouteMessage::default();

        message.header.table = RouteHeader::RT_TABLE_MAIN;
        message.header.protocol = RouteProtocol::Static;
        message.header.scope = RouteScope::Universe;
        message.header.kind = RouteType::Unicast;

        RouteAddRequest {
            handle,
            message,
            nl_msg_flags: 0,
            _phantom: Default::default(),
        }
    }

    /// Sets the input interface index.
    pub fn input_interface(mut self, index: u32) -> Self {
        self.message.attributes.push(RouteAttribute::Iif(index));
        self
    }

    /// Sets the output interface index.
    pub fn output_interface(mut self, index: u32) -> Self {
        self.message.attributes.push(RouteAttribute::Oif(index));
        self
    }

    /// Sets the route priority (metric)
    pub fn priority(mut self, priority: u32) -> Self {
        self.message
            .attributes
            .push(RouteAttribute::Priority(priority));
        self
    }

    /// Sets the route table.
    ///
    /// Default is main route table.
    #[deprecated(note = "Please use `table_id` instead")]
    pub fn table(mut self, table: u8) -> Self {
        self.message.header.table = table;
        self
    }

    /// Sets the route table ID.
    ///
    /// Default is main route table.
    pub fn table_id(mut self, table: u32) -> Self {
        if table > 255 {
            self.message.attributes.push(RouteAttribute::Table(table));
        } else {
            self.message.header.table = table as u8;
        }
        self
    }

    /// Sets the route protocol.
    ///
    /// Default is static route protocol.
    pub fn protocol(mut self, protocol: RouteProtocol) -> Self {
        self.message.header.protocol = protocol;
        self
    }

    /// Sets the route scope.
    ///
    /// Default is universe route scope.
    pub fn scope(mut self, scope: RouteScope) -> Self {
        self.message.header.scope = scope;
        self
    }

    /// Sets the route kind.
    ///
    /// Default is unicast route kind.
    pub fn kind(mut self, kind: RouteType) -> Self {
        self.message.header.kind = kind;
        self
    }

    /// Build an IP v4 route request
    pub fn v4(mut self) -> RouteAddRequest<Ipv4Addr> {
        self.message.header.address_family = AddressFamily::Inet;
        RouteAddRequest {
            handle: self.handle,
            message: self.message,
            nl_msg_flags: 0,
            _phantom: Default::default(),
        }
    }

    /// Build an IP v6 route request
    pub fn v6(mut self) -> RouteAddRequest<Ipv6Addr> {
        self.message.header.address_family = AddressFamily::Inet6;
        RouteAddRequest {
            handle: self.handle,
            message: self.message,
            nl_msg_flags: 0,
            _phantom: Default::default(),
        }
    }

    /// Replace existing matching route.
    pub fn replace(self) -> Self {
        Self {
            nl_msg_flags: NLM_F_REPLACE,
            ..self
        }
    }

    pub fn append(self) -> Self {
        Self {
            nl_msg_flags: NLM_F_APPEND,
            ..self
        }
    }

    /// Execute the request.
    pub async fn execute(self) -> Result<(), Error> {
        let RouteAddRequest {
            mut handle,
            message,
            mut nl_msg_flags,
            ..
        } = self;
        let mut req =
            NetlinkMessage::from(RouteNetlinkMessage::NewRoute(message));
        if (nl_msg_flags & NLM_F_REPLACE) == 0 {
            nl_msg_flags = NLM_F_EXCL;
        }
        req.header.flags =
            nl_msg_flags | NLM_F_REQUEST | NLM_F_ACK | NLM_F_CREATE;

        let mut response = handle.request(req)?;
        while let Some(message) = response.next().await {
            try_nl!(message);
        }
        Ok(())
    }

    /// Return a mutable reference to the request message.
    pub fn message_mut(&mut self) -> &mut RouteMessage {
        &mut self.message
    }
}

impl RouteAddRequest<Ipv4Addr> {
    /// Sets the source address prefix.
    pub fn source_prefix(mut self, addr: Ipv4Addr, prefix_length: u8) -> Self {
        self.message.header.source_prefix_length = prefix_length;
        self.message
            .attributes
            .push(RouteAttribute::Source(RouteAddress::Inet(addr)));
        self
    }

    /// Sets the preferred source address.
    pub fn pref_source(mut self, addr: Ipv4Addr) -> Self {
        self.message
            .attributes
            .push(RouteAttribute::PrefSource(RouteAddress::Inet(addr)));
        self
    }

    /// Sets the destination address prefix.
    pub fn destination_prefix(
        mut self,
        addr: Ipv4Addr,
        prefix_length: u8,
    ) -> Self {
        self.message.header.destination_prefix_length = prefix_length;
        self.message
            .attributes
            .push(RouteAttribute::Destination(RouteAddress::Inet(addr)));
        self
    }

    /// Sets the gateway (via) address.
    pub fn gateway(mut self, addr: Ipv4Addr) -> Self {
        self.message
            .attributes
            .push(RouteAttribute::Gateway(RouteAddress::Inet(addr)));
        self
    }
}

impl RouteAddRequest<Ipv6Addr> {
    /// Sets the source address prefix.
    pub fn source_prefix(mut self, addr: Ipv6Addr, prefix_length: u8) -> Self {
        self.message.header.source_prefix_length = prefix_length;
        self.message
            .attributes
            .push(RouteAttribute::Source(RouteAddress::Inet6(addr)));
        self
    }

    /// Sets the preferred source address.
    pub fn pref_source(mut self, addr: Ipv6Addr) -> Self {
        self.message
            .attributes
            .push(RouteAttribute::PrefSource(RouteAddress::Inet6(addr)));
        self
    }

    /// Sets the destination address prefix.
    pub fn destination_prefix(
        mut self,
        addr: Ipv6Addr,
        prefix_length: u8,
    ) -> Self {
        self.message.header.destination_prefix_length = prefix_length;
        self.message
            .attributes
            .push(RouteAttribute::Destination(RouteAddress::Inet6(addr)));
        self
    }

    /// Sets the gateway (via) address.
    pub fn gateway(mut self, addr: Ipv6Addr) -> Self {
        self.message
            .attributes
            .push(RouteAttribute::Gateway(RouteAddress::Inet6(addr)));
        self
    }
}
