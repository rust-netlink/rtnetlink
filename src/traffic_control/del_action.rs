// SPDX-License-Identifier: MIT

use futures::StreamExt;
use netlink_packet_core::{NetlinkMessage, NLM_F_ACK, NLM_F_REQUEST};
use netlink_packet_route::tc::{
    TcAction, TcActionMessage, TcActionMessageAttribute,
};
use netlink_packet_route::RouteNetlinkMessage;

use crate::{try_nl, Error, Handle};

/// A request to delete a traffic control action
#[must_use]
pub struct TrafficActionDelRequest {
    handle: Handle,
    message: TcActionMessage,
}

impl TrafficActionDelRequest {
    pub(crate) fn new(handle: Handle) -> Self {
        TrafficActionDelRequest {
            handle,
            message: TcActionMessage::default(),
        }
    }

    /// Specifies the action to delete
    pub fn action(mut self, action: TcAction) -> Self {
        self.message
            .attributes
            .push(TcActionMessageAttribute::Actions(vec![action]));
        self
    }

    /// Execute the request
    pub async fn execute(self) -> Result<(), Error> {
        let TrafficActionDelRequest {
            mut handle,
            message,
        } = self;

        let mut req = NetlinkMessage::from(
            RouteNetlinkMessage::DelTrafficAction(message),
        );
        req.header.flags = NLM_F_REQUEST | NLM_F_ACK;

        let mut response = handle.request(req)?;
        while let Some(message) = response.next().await {
            try_nl!(message)
        }
        Ok(())
    }

    /// Return a mutable reference to the request
    pub fn message_mut(&mut self) -> &mut TcActionMessage {
        &mut self.message
    }
}
