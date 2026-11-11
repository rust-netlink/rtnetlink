// SPDX-License-Identifier: MIT

#[macro_export]
macro_rules! try_rtnl {
    ($msg: expr, $message_type:path) => {{
        use netlink_packet_core::{NetlinkMessage, NetlinkPayload};
        use netlink_packet_route::RouteNetlinkMessage;
        use $crate::Error;

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
        use netlink_packet_core::NetlinkPayload;
        use $crate::Error;
        let message = $msg?;
        if let NetlinkPayload::Error(err) = message.payload {
            return Err(Error::NetlinkError(err));
        }
    }};
}
