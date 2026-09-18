// SPDX-License-Identifier: MIT

use std::net::IpAddr;

use futures_util::stream::StreamExt;
use netlink_packet_core::{
    NetlinkMessage, NLM_F_ACK, NLM_F_CREATE, NLM_F_EXCL, NLM_F_REQUEST,
};
use netlink_packet_route::{
    addrlabel::{AddrLabelAttribute, AddrLabelMessage},
    AddressFamily, RouteNetlinkMessage,
};

use crate::{try_nl, Error, Handle};

/// A request to create a new address label. This is equivalent to the
/// `ip addrlabel add` command.
#[derive(Debug, Clone)]
pub struct AddrLabelAddRequest {
    handle: Handle,
    message: AddrLabelMessage,
}

impl AddrLabelAddRequest {
    pub(crate) fn new(handle: Handle) -> Self {
        let mut message = AddrLabelMessage::default();
        // Kernel only registers `RTM_NEWADDRLABEL` handler for the IPv6
        // address family, hence default to IPv6 like iproute2 does.
        message.header.family = AddressFamily::Inet6;
        AddrLabelAddRequest { handle, message }
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

    /// Execute the request.
    pub async fn execute(self) -> Result<(), Error> {
        let AddrLabelAddRequest {
            mut handle,
            message,
        } = self;
        let mut req =
            NetlinkMessage::from(RouteNetlinkMessage::NewAddrLabel(message));
        req.header.flags =
            NLM_F_REQUEST | NLM_F_ACK | NLM_F_CREATE | NLM_F_EXCL;

        let mut response = handle.request(req)?;
        while let Some(message) = response.next().await {
            try_nl!(message);
        }

        Ok(())
    }

    pub fn message_mut(&mut self) -> &mut AddrLabelMessage {
        &mut self.message
    }
}
