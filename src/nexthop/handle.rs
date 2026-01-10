// SPDX-License-Identifier: MIT

use crate::{Handle, NexthopAddRequest, NexthopDelRequest, NexthopGetRequest};
use netlink_packet_route::nexthop::NexthopMessage;

#[derive(Clone, Debug)]
pub struct NexthopHandle(Handle);

impl NexthopHandle {
    pub fn new(handle: Handle) -> Self {
        NexthopHandle(handle)
    }

    /// List nexthops (equivalent to `ip nexthop show`)
    pub fn get(&self) -> NexthopGetRequest {
        NexthopGetRequest::new(self.0.clone())
    }

    /// Add a nexthop (equivalent to `ip nexthop add`)
    pub fn add(&self, message: NexthopMessage) -> NexthopAddRequest {
        NexthopAddRequest::new(self.0.clone(), message)
    }

    /// Delete a nexthop (equivalent to `ip nexthop del`)
    pub fn del(&self, id: u32) -> NexthopDelRequest {
        NexthopDelRequest::new(self.0.clone(), id)
    }
}
