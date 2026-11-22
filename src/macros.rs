// SPDX-License-Identifier: MIT

#[macro_export]
macro_rules! try_rtnl {
    ($msg: expr, $message_type:path) => {{
        use $crate::{
            packet_core::{NetlinkMessage, NetlinkPayload},
            packet_route::RouteNetlinkMessage,
            Error,
        };

        let msg = $msg?;
        let (header, payload) = msg.into_parts();
        match payload {
            NetlinkPayload::InnerMessage($message_type(msg)) => msg,
            NetlinkPayload::Error(err) => return Err(Error::NetlinkError(err)),
            _ => {
                return Err(Error::UnexpectedMessage(NetlinkMessage::new(
                    header, payload,
                )))
            }
        }
    }};
}

#[macro_export]
macro_rules! try_nl {
    ($msg: expr) => {{
        use $crate::{packet_core::NetlinkPayload, Error};
        let message = $msg?;
        if let NetlinkPayload::Error(err) = message.payload {
            return Err(Error::NetlinkError(err));
        }
    }};
}
