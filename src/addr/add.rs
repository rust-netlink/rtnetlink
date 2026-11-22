// SPDX-License-Identifier: MIT

use futures_util::stream::StreamExt;
use netlink_packet_core::{
    NetlinkMessage, NLM_F_ACK, NLM_F_CREATE, NLM_F_EXCL, NLM_F_REPLACE,
    NLM_F_REQUEST,
};
use netlink_packet_route::{address::AddressMessage, RouteNetlinkMessage};

use crate::{try_nl, Error, Handle};

/// A request to create a new address. This is equivalent to the `ip address
/// add` commands.
#[derive(Debug, Clone)]
pub struct AddressAddRequest {
    handle: Handle,
    message: AddressMessage,
    replace: bool,
}

impl AddressAddRequest {
    pub(crate) fn new(handle: Handle, message: AddressMessage) -> Self {
        AddressAddRequest {
            handle,
            message,
            replace: false,
        }
    }

    /// Replace existing matching address.
    pub fn replace(self) -> Self {
        Self {
            replace: true,
            ..self
        }
    }

    /// Execute the request.
    pub async fn execute(self) -> Result<(), Error> {
        let AddressAddRequest {
            mut handle,
            message,
            replace,
        } = self;
        let mut req =
            NetlinkMessage::from(RouteNetlinkMessage::NewAddress(message));
        let replace = if replace { NLM_F_REPLACE } else { NLM_F_EXCL };
        req.header.flags = NLM_F_REQUEST | NLM_F_ACK | replace | NLM_F_CREATE;

        let mut response = handle.request(req)?;
        while let Some(message) = response.next().await {
            try_nl!(message);
        }
        Ok(())
    }

    /// Return a mutable reference to the request message.
    pub fn message_mut(&mut self) -> &mut AddressMessage {
        &mut self.message
    }
}
