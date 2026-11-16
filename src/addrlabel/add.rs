// SPDX-License-Identifier: MIT

use futures_util::stream::StreamExt;
use netlink_packet_core::{
    NetlinkMessage, NLM_F_ACK, NLM_F_CREATE, NLM_F_EXCL, NLM_F_REQUEST,
};
use netlink_packet_route::{addrlabel::AddrLabelMessage, RouteNetlinkMessage};

use crate::{try_nl, Error, Handle};

/// A request to create a new address label. This is equivalent to the
/// `ip addrlabel add` command.
#[derive(Debug, Clone)]
pub struct AddrLabelAddRequest {
    handle: Handle,
    message: AddrLabelMessage,
}

impl AddrLabelAddRequest {
    pub(crate) fn new(handle: Handle, message: AddrLabelMessage) -> Self {
        AddrLabelAddRequest { handle, message }
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
