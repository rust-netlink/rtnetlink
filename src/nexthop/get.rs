// SPDX-License-Identifier: MIT

use futures_util::{
    future::{self, Either},
    stream::{Stream, StreamExt},
    FutureExt,
};
use netlink_packet_core::{NetlinkMessage, NLM_F_DUMP, NLM_F_REQUEST};
use netlink_packet_route::{nexthop::NexthopMessage, RouteNetlinkMessage};

use crate::{try_rtnl, Error, Handle};

/// A request to get nexthops. This is equivalent to the `ip nexthop show` commands.
#[derive(Debug)]
pub struct NexthopGetRequest {
    handle: Handle,
    message: NexthopMessage,
}

impl NexthopGetRequest {
    pub(crate) fn new(handle: Handle) -> Self {
        NexthopGetRequest {
            handle,
            message: NexthopMessage::default(),
        }
    }

    /// Execute the request.
    pub fn execute(self) -> impl Stream<Item = Result<NexthopMessage, Error>> {
        let NexthopGetRequest {
            mut handle,
            message,
        } = self;
        let mut req =
            NetlinkMessage::from(RouteNetlinkMessage::GetNexthop(message));
        req.header.flags = NLM_F_REQUEST | NLM_F_DUMP;

        match handle.request(req) {
            Ok(response) => Either::Left(response.map(move |msg| {
                Ok(try_rtnl!(msg, RouteNetlinkMessage::NewNexthop))
            })),
            Err(e) => Either::Right(
                future::err::<NexthopMessage, Error>(e).into_stream(),
            ),
        }
    }

    pub fn message_mut(&mut self) -> &mut NexthopMessage {
        &mut self.message
    }
}
