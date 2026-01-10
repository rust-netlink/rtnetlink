// SPDX-License-Identifier: MIT

use crate::{try_nl, Error, Handle};
use futures_util::stream::StreamExt;
use netlink_packet_core::{NetlinkMessage, NLM_F_ACK, NLM_F_REQUEST};
use netlink_packet_route::{
    nexthop::{NexthopAttribute, NexthopMessage},
    RouteNetlinkMessage,
};

/// A request to delete a nexthop. This is equivalent to the `ip nexthop del`
/// commands.
#[derive(Debug)]
pub struct NexthopDelRequest {
    handle: Handle,
    message: NexthopMessage,
}

impl NexthopDelRequest {
    pub(crate) fn new(handle: Handle, id: u32) -> Self {
        let mut message = NexthopMessage::default();
        message.nlas.push(NexthopAttribute::Id(id));
        NexthopDelRequest { handle, message }
    }

    /// Execute the request.
    pub async fn execute(self) -> Result<(), Error> {
        let NexthopDelRequest {
            mut handle,
            message,
        } = self;
        let mut req =
            NetlinkMessage::from(RouteNetlinkMessage::DelNexthop(message));
        req.header.flags = NLM_F_REQUEST | NLM_F_ACK;

        let mut response = handle.request(req)?;
        while let Some(message) = response.next().await {
            try_nl!(message);
        }
        Ok(())
    }

    pub fn message_mut(&mut self) -> &mut NexthopMessage {
        &mut self.message
    }
}
