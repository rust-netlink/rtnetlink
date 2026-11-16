// SPDX-License-Identifier: MIT

use futures_util::{
    future::{self, Either},
    stream::{Stream, StreamExt},
    FutureExt,
};
use netlink_packet_core::{NetlinkMessage, NLM_F_DUMP, NLM_F_REQUEST};
use netlink_packet_route::{
    stats::{StatsFilterMask, StatsMessage},
    RouteNetlinkMessage,
};

use crate::{try_rtnl, Error, Handle};

#[derive(Debug, Clone)]
pub struct AfstatsRequest {
    handle: Handle,
    message: StatsMessage,
}

impl AfstatsRequest {
    pub(crate) fn new(handle: Handle) -> Self {
        let mut message = StatsMessage::default();
        message.header.filter_mask = StatsFilterMask::AfSpec;
        AfstatsRequest { handle, message }
    }

    /// Return a mutable reference to the request message.
    pub fn message_mut(&mut self) -> &mut StatsMessage {
        &mut self.message
    }

    /// Filter by link index
    pub fn match_index(mut self, index: u32) -> Self {
        self.message.header.ifindex = index;
        self
    }

    /// Execute the request and return a stream of stats messages
    pub fn execute(
        self,
    ) -> impl Stream<Item = Result<StatsMessage, Error>> + Send {
        let AfstatsRequest {
            mut handle,
            message,
        } = self;

        let mut req =
            NetlinkMessage::from(RouteNetlinkMessage::GetStats(message));
        req.header.flags = NLM_F_REQUEST | NLM_F_DUMP;

        match handle.request(req) {
            Ok(response) => Either::Left(response.map(move |msg| {
                Ok(try_rtnl!(msg, RouteNetlinkMessage::NewStats))
            })),
            Err(e) => Either::Right(
                future::err::<StatsMessage, Error>(e).into_stream(),
            ),
        }
    }
}
