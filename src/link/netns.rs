// SPDX-License-Identifier: MIT

use std::os::fd::AsRawFd;

use futures_util::StreamExt;
use netlink_packet_core::{NetlinkPayload, NLM_F_ACK, NLM_F_REQUEST};
use netlink_packet_route::{
    nsid::{NsidAttribute, NsidHeader, NsidMessage},
    RouteNetlinkMessage,
};

use crate::{Error, Handle};

/// Request to resolve a network namespace name to its nsid.
///
/// Equivalent to iproute2's `netns_id_from_name()`.
pub struct LinkGetNsidHandle {
    handle: Handle,
    name: String,
}

impl LinkGetNsidHandle {
    pub(crate) fn new(handle: Handle, name: String) -> Self {
        LinkGetNsidHandle { handle, name }
    }

    /// Execute the request. Returns `Some(nsid)` if the namespace has an nsid
    /// assigned, or `None` if it does not (`NETNSA_NSID_NOT_ASSIGNED`).
    pub async fn execute(mut self) -> Result<Option<i32>, Error> {
        let file = std::fs::File::open(format!("/run/netns/{}", self.name))
            .map_err(|e| Error::NamespaceError(e.to_string()))?;
        let fd = file.as_raw_fd();

        let mut msg = NsidMessage::default();
        msg.header = NsidHeader::default();
        msg.attributes = vec![NsidAttribute::Fd(fd as u32)];

        let mut req = netlink_packet_core::NetlinkMessage::from(
            RouteNetlinkMessage::GetNsId(msg),
        );
        req.header.flags = NLM_F_REQUEST | NLM_F_ACK;
        let mut resp_stream =
            self.handle.request(req).map_err(|_| Error::RequestFailed)?;

        while let Some(resp) = resp_stream.next().await {
            if let NetlinkPayload::Error(err) = resp.payload {
                return Err(Error::NetlinkError(err));
            }
            if let NetlinkPayload::InnerMessage(RouteNetlinkMessage::NewNsId(
                nsid_msg,
            )) = resp.payload
            {
                for attr in nsid_msg.attributes {
                    if let NsidAttribute::Id(id) = attr {
                        if id >= 0 {
                            return Ok(Some(id));
                        }
                    }
                }
            }
        }

        Ok(None)
    }
}

/// Request to assign a new nsid to a network namespace.
///
/// Equivalent to iproute2's `set_netns_id_from_name()`.
pub struct LinkAssignNsidHandle {
    handle: Handle,
    name: String,
    nsid: i32,
}

impl LinkAssignNsidHandle {
    pub(crate) fn new(handle: Handle, name: String, nsid: i32) -> Self {
        LinkAssignNsidHandle { handle, name, nsid }
    }

    /// Execute the request. Sends `RTM_NEWNSID` to assign the nsid to the
    /// namespace. Use `nsid = -1` for automatic assignment.
    pub async fn execute(mut self) -> Result<(), Error> {
        let file = std::fs::File::open(format!("/run/netns/{}", self.name))
            .map_err(|e| Error::NamespaceError(e.to_string()))?;
        let fd = file.as_raw_fd();

        let mut msg = NsidMessage::default();
        msg.header = NsidHeader::default();
        msg.attributes =
            vec![NsidAttribute::Fd(fd as u32), NsidAttribute::Id(self.nsid)];

        let mut req = netlink_packet_core::NetlinkMessage::from(
            RouteNetlinkMessage::NewNsId(msg),
        );
        req.header.flags = NLM_F_REQUEST | NLM_F_ACK;
        let mut resp_stream =
            self.handle.request(req).map_err(|_| Error::RequestFailed)?;

        while let Some(resp) = resp_stream.next().await {
            if let NetlinkPayload::Error(err) = resp.payload {
                return Err(Error::NetlinkError(err));
            }
        }

        Ok(())
    }
}
