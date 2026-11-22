// SPDX-License-Identifier: MIT

use futures_util::stream::StreamExt;
use netlink_packet_core::{NetlinkMessage, NLM_F_ACK, NLM_F_REQUEST};
use netlink_packet_route::{tc::TcMessage, RouteNetlinkMessage};

use crate::{try_nl, Error, Handle};

#[derive(Debug, Clone)]
pub struct TrafficFilterDelRequest {
    handle: Handle,
    message: TcMessage,
    flags: u16,
}

impl TrafficFilterDelRequest {
    pub(crate) fn new(handle: Handle, message: TcMessage) -> Self {
        Self {
            handle,
            message,
            flags: NLM_F_REQUEST | NLM_F_ACK,
        }
    }

    /// Return a mutable reference to the request message.
    pub fn message_mut(&mut self) -> &mut TcMessage {
        &mut self.message
    }

    /// Execute the request
    pub async fn execute(self) -> Result<(), Error> {
        let Self {
            mut handle,
            message,
            flags,
        } = self;

        let mut req = NetlinkMessage::from(
            RouteNetlinkMessage::DelTrafficFilter(message),
        );
        req.header.flags = flags;

        let mut response = handle.request(req)?;
        if let Some(message) = response.next().await {
            try_nl!(message);
        }

        Ok(())
    }
}
