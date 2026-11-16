// SPDX-License-Identifier: MIT

use futures_util::stream::StreamExt;
use netlink_packet_core::{NetlinkMessage, NLM_F_ACK, NLM_F_REQUEST};
use netlink_packet_route::{addrlabel::AddrLabelMessage, RouteNetlinkMessage};

use crate::{try_nl, Error, Handle};

/// A request to delete an existing address label. This is equivalent to the
/// `ip addrlabel del` command.
#[derive(Debug, Clone)]
pub struct AddrLabelDelRequest {
    handle: Handle,
    message: AddrLabelMessage,
}

impl AddrLabelDelRequest {
    pub(crate) fn new(handle: Handle, message: AddrLabelMessage) -> Self {
        AddrLabelDelRequest { handle, message }
    }

    /// Execute the request.
    pub async fn execute(self) -> Result<(), Error> {
        let AddrLabelDelRequest {
            mut handle,
            message,
        } = self;
        let mut req =
            NetlinkMessage::from(RouteNetlinkMessage::DelAddrLabel(message));
        req.header.flags = NLM_F_REQUEST | NLM_F_ACK;

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
