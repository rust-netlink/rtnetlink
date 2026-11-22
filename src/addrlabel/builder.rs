// SPDX-License-Identifier: MIT

use std::net::IpAddr;

use netlink_packet_route::{
    addrlabel::{AddrLabelAttribute, AddrLabelMessage},
    AddressFamily,
};

/// Helper struct for building [AddrLabelMessage].
#[derive(Debug, Clone)]
pub struct AddrLabelMessageBuilder {
    message: AddrLabelMessage,
}

impl Default for AddrLabelMessageBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl AddrLabelMessageBuilder {
    /// Create a new [AddrLabelMessageBuilder].
    ///
    /// The default address family is `AddressFamily::Inet6`, because the
    /// kernel only registers the `RTM_NEWADDRLABEL` handler for the IPv6
    /// address family, matching `ip addrlabel add`.
    pub fn new() -> Self {
        let mut message = AddrLabelMessage::default();
        message.header.family = AddressFamily::Inet6;
        Self { message }
    }

    /// Sets the address family of the address label.
    pub fn family(mut self, family: AddressFamily) -> Self {
        self.message.header.family = family;
        self
    }

    /// Sets the address of the address label.
    pub fn address(mut self, address: IpAddr) -> Self {
        self.message
            .attributes
            .push(AddrLabelAttribute::Address(address));
        self
    }

    /// Sets the prefix length of the address label.
    pub fn prefix_len(mut self, prefix_len: u8) -> Self {
        self.message.header.prefix_len = prefix_len;
        self
    }

    /// Sets the interface index of the address label.
    pub fn index(mut self, index: u32) -> Self {
        self.message.header.index = index;
        self
    }

    /// Sets the label of the address label.
    pub fn label(mut self, label: u32) -> Self {
        self.message
            .attributes
            .push(AddrLabelAttribute::Label(label));
        self
    }

    /// Return a mutable reference to the message being built.
    pub fn get_mut(&mut self) -> &mut AddrLabelMessage {
        &mut self.message
    }

    /// Append an arbitrary [AddrLabelAttribute].
    pub fn append_attribute(mut self, attribute: AddrLabelAttribute) -> Self {
        self.message.attributes.push(attribute);
        self
    }

    /// Builds [AddrLabelMessage].
    pub fn build(self) -> AddrLabelMessage {
        self.message
    }
}
