// SPDX-License-Identifier: MIT

use futures::{
    future::{self, Either},
    stream::{Stream, StreamExt},
    FutureExt,
};
use netlink_packet_core::{NetlinkMessage, NLM_F_DUMP, NLM_F_REQUEST};
use netlink_packet_route::{
    route::RouteHeader,
    rule::{RuleAction, RuleMessage},
    RouteNetlinkMessage,
};

use crate::{try_rtnl, Error, Handle, IpVersion};

#[derive(Debug, Clone)]
pub struct RuleGetRequest {
    handle: Handle,
    message: RuleMessage,
}

impl RuleGetRequest {
    pub(crate) fn new(handle: Handle, ip_version: IpVersion) -> Self {
        let mut message = RuleMessage::default();
        message.header.family = ip_version.family();

        message.header.dst_len = 0;
        message.header.src_len = 0;
        message.header.tos = 0;
        message.header.action = RuleAction::Unspec;
        message.header.table = RouteHeader::RT_TABLE_UNSPEC;

        RuleGetRequest { handle, message }
    }

    pub fn message_mut(&mut self) -> &mut RuleMessage {
        &mut self.message
    }

    pub fn execute(self) -> impl Stream<Item = Result<RuleMessage, Error>> {
        let RuleGetRequest {
            mut handle,
            message,
        } = self;

        let mut req =
            NetlinkMessage::from(RouteNetlinkMessage::GetRule(message));
        req.header.flags = NLM_F_REQUEST | NLM_F_DUMP;

        match handle.request(req) {
            Ok(response) => Either::Left(response.map(move |msg| {
                Ok(try_rtnl!(msg, RouteNetlinkMessage::NewRule))
            })),
            Err(e) => Either::Right(
                future::err::<RuleMessage, Error>(e).into_stream(),
            ),
        }
    }
}
