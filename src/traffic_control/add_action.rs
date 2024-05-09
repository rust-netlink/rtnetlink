// SPDX-License-Identifier: MIT

use futures::future::Either;
use futures::{future, FutureExt, StreamExt, TryStream};
use netlink_packet_core::{
    NetlinkMessage, NLM_F_ACK, NLM_F_EXCL, NLM_F_REQUEST,
};
use netlink_packet_route::tc::{
    TcAction, TcActionMessage, TcActionMessageAttribute,
};
use netlink_packet_route::RouteNetlinkMessage;
use nix::libc::RTM_NEWACTION;

use crate::{try_rtnl, Error, Handle};

/// A request to add a new traffic control action
#[derive(Debug, Clone)]
#[must_use = "builder"]
pub struct TrafficActionNewRequest {
    handle: Handle,
    message: TcActionMessage,
}

impl TrafficActionNewRequest {
    pub(crate) fn new(handle: Handle) -> Self {
        Self {
            handle,
            message: TcActionMessage::default(),
        }
    }

    /// Specifies the action to add
    pub fn action(mut self, action: TcAction) -> Self {
        self.message
            .attributes
            .push(TcActionMessageAttribute::Actions(vec![action]));
        self
    }

    /// Execute the request
    #[must_use = "builder"]
    pub fn execute(
        self,
    ) -> impl TryStream<Ok = TcActionMessage, Error = Error> {
        let Self {
            mut handle,
            message,
        } = self;

        let mut req = NetlinkMessage::from(
            RouteNetlinkMessage::NewTrafficAction(message),
        );
        req.header.message_type = RTM_NEWACTION;
        req.header.flags = NLM_F_REQUEST | NLM_F_ACK | NLM_F_EXCL;

        match handle.request(req) {
            Ok(response) => Either::Left(response.map(move |msg| {
                Ok(try_rtnl!(msg, RouteNetlinkMessage::NewTrafficAction))
            })),
            Err(err) => Either::Right(
                future::err::<TcActionMessage, Error>(err).into_stream(),
            ),
        }
    }
}
