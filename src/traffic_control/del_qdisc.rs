// SPDX-License-Identifier: MIT

use futures_util::stream::StreamExt;
use netlink_packet_core::{NetlinkMessage, NLM_F_ACK, NLM_F_REQUEST};
use netlink_packet_route::{
    tc::{TcAttribute, TcHandle, TcMessage},
    RouteNetlinkMessage,
};

use crate::{try_nl, Error, Handle};

#[derive(Debug, Clone)]
pub struct QDiscDelRequest {
    handle: Handle,
    message: TcMessage,
}

impl QDiscDelRequest {
    pub(crate) fn new(handle: Handle, message: TcMessage) -> Self {
        QDiscDelRequest { handle, message }
    }

    // Execute the request
    pub async fn execute(self) -> Result<(), Error> {
        let QDiscDelRequest {
            mut handle,
            message,
        } = self;

        let mut req = NetlinkMessage::from(
            RouteNetlinkMessage::DelQueueDiscipline(message),
        );
        req.header.flags = NLM_F_REQUEST | NLM_F_ACK;

        let mut response = handle.request(req)?;
        while let Some(message) = response.next().await {
            try_nl!(message)
        }
        Ok(())
    }

    /// Return a mutable reference to the request
    pub fn message_mut(&mut self) -> &mut TcMessage {
        &mut self.message
    }

    /// Set handle
    pub fn handle(mut self, major: u16, minor: u16) -> Self {
        self.message.header.handle = TcHandle { major, minor };
        self
    }

    /// Set parent to root
    pub fn root(mut self) -> Self {
        self.message.header.parent = TcHandle::ROOT;
        self
    }

    /// Set parent
    pub fn parent(mut self, parent: u32) -> Self {
        self.message.header.parent = parent.into();
        self
    }

    /// Set ingress qdisc
    pub fn ingress(mut self) -> Self {
        self.message.header.parent = TcHandle::INGRESS;
        self.message.header.handle = TcHandle::from(0xffff0000);
        self.message
            .attributes
            .push(TcAttribute::Kind("ingress".to_string()));
        self
    }
}
