// SPDX-License-Identifier: MIT

use futures_util::{
    future::{self, Either},
    stream::{Stream, StreamExt},
    FutureExt,
};
use netlink_packet_core::{NetlinkMessage, NLM_F_DUMP, NLM_F_REQUEST};
use netlink_packet_route::{addrlabel::AddrLabelMessage, RouteNetlinkMessage};

use crate::{try_rtnl, Error, Handle, IpVersion};

#[derive(Debug, Clone)]
pub struct AddrLabelGetRequest {
    handle: Handle,
    message: AddrLabelMessage,
}

impl AddrLabelGetRequest {
    pub(crate) fn new(handle: Handle, ip_version: IpVersion) -> Self {
        let mut message = AddrLabelMessage::default();
        message.header.family = ip_version.family();
        AddrLabelGetRequest { handle, message }
    }

    pub fn message_mut(&mut self) -> &mut AddrLabelMessage {
        &mut self.message
    }

    /// Execute the request
    pub fn execute(
        self,
    ) -> impl Stream<Item = Result<AddrLabelMessage, Error>> {
        let AddrLabelGetRequest {
            mut handle,
            message,
        } = self;

        let mut req =
            NetlinkMessage::from(RouteNetlinkMessage::GetAddrLabel(message));
        req.header.flags = NLM_F_REQUEST | NLM_F_DUMP;

        match handle.request(req) {
            Ok(response) => Either::Left(response.map(move |msg| {
                Ok(try_rtnl!(msg, RouteNetlinkMessage::NewAddrLabel))
            })),
            Err(e) => Either::Right(
                future::err::<AddrLabelMessage, Error>(e).into_stream(),
            ),
        }
    }
}
