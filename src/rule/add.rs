// SPDX-License-Identifier: MIT

use futures_util::stream::StreamExt;
use netlink_packet_core::{
    NetlinkMessage, NLM_F_ACK, NLM_F_CREATE, NLM_F_EXCL, NLM_F_REPLACE,
    NLM_F_REQUEST,
};
use netlink_packet_route::{rule::RuleMessage, RouteNetlinkMessage};

use crate::{try_nl, Error, Handle};

/// A request to create a new rule. This is equivalent to the `ip rule add`
/// command.
#[derive(Debug, Clone)]
pub struct RuleAddRequest {
    handle: Handle,
    message: RuleMessage,
    replace: bool,
}

impl RuleAddRequest {
    pub(crate) fn new(handle: Handle, message: RuleMessage) -> Self {
        RuleAddRequest {
            handle,
            message,
            replace: false,
        }
    }

    /// Replace existing matching rule.
    pub fn replace(self) -> Self {
        Self {
            replace: true,
            ..self
        }
    }

    /// Execute the request.
    pub async fn execute(self) -> Result<(), Error> {
        let RuleAddRequest {
            mut handle,
            message,
            replace,
        } = self;
        let mut req =
            NetlinkMessage::from(RouteNetlinkMessage::NewRule(message));
        let replace = if replace { NLM_F_REPLACE } else { NLM_F_EXCL };
        req.header.flags = NLM_F_REQUEST | NLM_F_ACK | replace | NLM_F_CREATE;

        let mut response = handle.request(req)?;
        while let Some(message) = response.next().await {
            try_nl!(message);
        }

        Ok(())
    }

    pub fn message_mut(&mut self) -> &mut RuleMessage {
        &mut self.message
    }
}
