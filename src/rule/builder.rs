// SPDX-License-Identifier: MIT

use std::{
    marker::PhantomData,
    net::{Ipv4Addr, Ipv6Addr},
};

use netlink_packet_route::{
    route::RouteHeader,
    rule::{RuleAction, RuleAttribute, RuleMessage},
    AddressFamily,
};

/// Helper struct for building [RuleMessage].
#[derive(Debug, Clone)]
pub struct RuleMessageBuilder<T> {
    message: RuleMessage,
    _phantom: PhantomData<T>,
}

impl<T> RuleMessageBuilder<T> {
    /// Create a new [RuleMessageBuilder] without specifying the address
    /// family.
    fn new_no_address_family() -> Self {
        let mut message = RuleMessage::default();

        message.header.table = RouteHeader::RT_TABLE_MAIN;
        message.header.action = RuleAction::Unspec;

        RuleMessageBuilder {
            message,
            _phantom: PhantomData,
        }
    }

    /// Sets the input interface name.
    pub fn input_interface(mut self, ifname: impl Into<String>) -> Self {
        self.message
            .attributes
            .push(RuleAttribute::Iifname(ifname.into()));
        self
    }

    /// Sets the output interface name.
    pub fn output_interface(mut self, ifname: impl Into<String>) -> Self {
        self.message
            .attributes
            .push(RuleAttribute::Oifname(ifname.into()));
        self
    }

    /// Sets the rule table.
    ///
    /// Default is main rule table.
    #[deprecated(note = "Please use `table_id` instead")]
    pub fn table(mut self, table: u8) -> Self {
        self.message.header.table = table;
        self
    }

    /// Sets the rule table ID.
    ///
    /// Default is main rule table.
    pub fn table_id(mut self, table: u32) -> Self {
        if table > 255 {
            self.message.attributes.push(RuleAttribute::Table(table));
        } else {
            self.message.header.table = table as u8;
        }
        self
    }

    /// Set the tos.
    pub fn tos(mut self, tos: u8) -> Self {
        self.message.header.tos = tos;
        self
    }

    /// Set action.
    pub fn action(mut self, action: RuleAction) -> Self {
        self.message.header.action = action;
        self
    }

    /// Set the priority.
    pub fn priority(mut self, priority: u32) -> Self {
        self.message
            .attributes
            .push(RuleAttribute::Priority(priority));
        self
    }

    /// Set the fwmark
    pub fn fw_mark(mut self, fw_mark: u32) -> Self {
        self.message.attributes.push(RuleAttribute::FwMark(fw_mark));
        self
    }

    /// Return a mutable reference to the message being built.
    pub fn get_mut(&mut self) -> &mut RuleMessage {
        &mut self.message
    }

    /// Append an arbitrary [RuleAttribute].
    pub fn append_attribute(mut self, attribute: RuleAttribute) -> Self {
        self.message.attributes.push(attribute);
        self
    }

    /// Builds [RuleMessage].
    pub fn build(self) -> RuleMessage {
        self.message
    }
}

impl Default for RuleMessageBuilder<()> {
    fn default() -> Self {
        Self::new()
    }
}

impl RuleMessageBuilder<()> {
    /// Create a new [RuleMessageBuilder] without specifying the address
    /// family.
    pub fn new() -> Self {
        Self::new_no_address_family()
    }
}

impl Default for RuleMessageBuilder<Ipv4Addr> {
    fn default() -> Self {
        Self::new()
    }
}

impl RuleMessageBuilder<Ipv4Addr> {
    /// Create a new [RuleMessageBuilder] for IPv4 rules.
    pub fn new() -> Self {
        let mut builder = Self::new_no_address_family();
        builder.message.header.family = AddressFamily::Inet;
        builder
    }

    /// Sets the source address prefix.
    pub fn source_prefix(mut self, addr: Ipv4Addr, prefix_length: u8) -> Self {
        self.message.header.src_len = prefix_length;
        self.message
            .attributes
            .push(RuleAttribute::Source(addr.into()));
        self
    }

    /// Sets the destination address prefix.
    pub fn destination_prefix(
        mut self,
        addr: Ipv4Addr,
        prefix_length: u8,
    ) -> Self {
        self.message.header.dst_len = prefix_length;
        self.message
            .attributes
            .push(RuleAttribute::Destination(addr.into()));
        self
    }
}

impl Default for RuleMessageBuilder<Ipv6Addr> {
    fn default() -> Self {
        Self::new()
    }
}

impl RuleMessageBuilder<Ipv6Addr> {
    /// Create a new [RuleMessageBuilder] for IPv6 rules.
    pub fn new() -> Self {
        let mut builder = Self::new_no_address_family();
        builder.message.header.family = AddressFamily::Inet6;
        builder
    }

    /// Sets the source address prefix.
    pub fn source_prefix(mut self, addr: Ipv6Addr, prefix_length: u8) -> Self {
        self.message.header.src_len = prefix_length;
        self.message
            .attributes
            .push(RuleAttribute::Source(addr.into()));
        self
    }

    /// Sets the destination address prefix.
    pub fn destination_prefix(
        mut self,
        addr: Ipv6Addr,
        prefix_length: u8,
    ) -> Self {
        self.message.header.dst_len = prefix_length;
        self.message
            .attributes
            .push(RuleAttribute::Destination(addr.into()));
        self
    }
}
