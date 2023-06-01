// SPDX-License-Identifier: MIT

use crate::{Handle, RouteAddRequest, RouteDelRequest, RouteGetRequest};
use netlink_packet_route::RouteMessage;

pub struct RouteHandle(Handle);

impl RouteHandle {
    pub fn new(handle: Handle) -> Self {
        RouteHandle(handle)
    }

    /// Retrieve the routing table entry that would be used to route to
    /// the given destination address (or the entire routing table if
    /// no destination address is specified)
    pub fn get(&self) -> RouteGetRequest {
        RouteGetRequest::new(self.0.clone())
    }

    /// Add an routing table entry (equivalent to `ip route add`)
    pub fn add(&self) -> RouteAddRequest {
        RouteAddRequest::new(self.0.clone())
    }

    /// Delete the given routing table entry (equivalent to `ip route del`)
    pub fn del(&self, route: RouteMessage) -> RouteDelRequest {
        RouteDelRequest::new(self.0.clone(), route)
    }
}
