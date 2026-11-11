// SPDX-License-Identifier: MIT

use std::fmt;

use netlink_packet_core::{ErrorMessage, NetlinkMessage};
use netlink_packet_route::RouteNetlinkMessage;
use netlink_proto::NetlinkProtoError;

#[derive(Debug)]
pub enum Error {
    UnexpectedMessage(NetlinkMessage<RouteNetlinkMessage>),

    NetlinkError(ErrorMessage),

    NetlinkProtoError(NetlinkProtoError),

    RequestFailed,

    NamespaceError(String),

    InvalidHardwareAddress(Vec<u8>),

    InvalidIp(Vec<u8>),

    InvalidAddress(Vec<u8>, Vec<u8>),

    InvalidNla(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedMessage(msg) => {
                write!(f, "Received an unexpected message {msg:?}")
            }
            Self::NetlinkError(err) => {
                write!(f, "Received a netlink error message {err}")
            }
            Self::NetlinkProtoError(err) => {
                write!(f, "Netlink protocol error: {err}")
            }
            Self::RequestFailed => write!(f, "A netlink request failed"),
            Self::NamespaceError(err) => {
                write!(f, "Namespace error {err}")
            }
            Self::InvalidHardwareAddress(addr) => write!(
                f,
                "Received a link message (RTM_GETLINK, RTM_NEWLINK, \
                 RTM_SETLINK or RTMGETLINK) with an invalid hardware address \
                 attribute: {addr:?}."
            ),
            Self::InvalidIp(addr) => {
                write!(f, "Failed to parse an IP address: {addr:?}")
            }
            Self::InvalidAddress(ip, mask) => write!(
                f,
                "Failed to parse a network address (IP and mask): {ip:?}/{mask:?}"
            ),
            Self::InvalidNla(nla) => {
                write!(f, "Attempting to set and Invalid NLA: {nla}")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::NetlinkProtoError(err) => Some(err),
            _ => None,
        }
    }
}
