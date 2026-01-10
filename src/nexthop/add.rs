// SPDX-License-Identifier: MIT

use crate::{try_nl, Error, Handle};
use futures_util::stream::StreamExt;
use netlink_packet_core::{
    NetlinkMessage, NLM_F_ACK, NLM_F_CREATE, NLM_F_EXCL, NLM_F_REPLACE,
    NLM_F_REQUEST,
};
use netlink_packet_route::{nexthop::NexthopMessage, RouteNetlinkMessage};

/// A request to create a new nexthop. This is equivalent to the `ip nexthop add`
/// commands.
#[derive(Debug)]
pub struct NexthopAddRequest {
    handle: Handle,
    message: NexthopMessage,
    replace: bool,
}

impl NexthopAddRequest {
    pub(crate) fn new(handle: Handle, message: NexthopMessage) -> Self {
        NexthopAddRequest {
            handle,
            message,
            replace: false,
        }
    }

    /// Replace existing matching nexthop.
    pub fn replace(self) -> Self {
        Self {
            replace: true,
            ..self
        }
    }

    /// Execute the request.
    pub async fn execute(self) -> Result<(), Error> {
        let NexthopAddRequest {
            mut handle, // Need mut for handle.request
            message,
            replace,
        } = self;
        let mut req =
            NetlinkMessage::from(RouteNetlinkMessage::NewNexthop(message));
        let replace = if replace { NLM_F_REPLACE } else { NLM_F_EXCL };
        req.header.flags = NLM_F_REQUEST | NLM_F_ACK | replace | NLM_F_CREATE;

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
