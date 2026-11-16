// SPDX-License-Identifier: MIT

use netlink_packet_route::{
    tc::{TcAttribute, TcHandle, TcMessage},
    AddressFamily,
};

/// Helper struct for building [TcMessage].
#[derive(Debug, Clone, Default)]
pub struct TcMessageBuilder {
    message: TcMessage,
}

impl TcMessageBuilder {
    /// Create a new [TcMessageBuilder].
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [TcMessageBuilder] with the given interface index.
    pub fn with_index(index: i32) -> Self {
        Self::new().index(index)
    }

    /// Sets the interface index.
    pub fn index(mut self, index: i32) -> Self {
        self.message.header.index = index;
        self
    }

    /// Sets the address family.
    pub fn family(mut self, family: AddressFamily) -> Self {
        self.message.header.family = family;
        self
    }

    /// Sets the qdisc handle.
    pub fn handle(mut self, major: u16, minor: u16) -> Self {
        self.message.header.handle = TcHandle::new(major, minor);
        self
    }

    /// Sets the parent.
    pub fn parent(mut self, parent: u32) -> Self {
        self.message.header.parent = parent.into();
        self
    }

    /// Sets the parent to root.
    pub fn root(mut self) -> Self {
        self.message.header.parent = TcHandle::ROOT;
        self
    }

    /// Sets the parent to ingress.
    pub fn ingress(mut self) -> Self {
        self.message.header.parent =
            TcHandle::new(0xffff, TcHandle::MIN_INGRESS);
        self
    }

    /// Sets the parent to egress.
    pub fn egress(mut self) -> Self {
        self.message.header.parent =
            TcHandle::new(0xffff, TcHandle::MIN_EGRESS);
        self
    }

    /// Return a mutable reference to the message being built.
    pub fn get_mut(&mut self) -> &mut TcMessage {
        &mut self.message
    }

    /// Append an arbitrary [TcAttribute].
    pub fn append_attribute(mut self, attribute: TcAttribute) -> Self {
        self.message.attributes.push(attribute);
        self
    }

    /// Builds [TcMessage].
    pub fn build(self) -> TcMessage {
        self.message
    }
}
