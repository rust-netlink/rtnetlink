// SPDX-License-Identifier: MIT

use netlink_packet_route::neighbour::NeighbourMessage;

use crate::{
    Handle, NeighbourAddRequest, NeighbourDelRequest, NeighbourGetRequest,
};

pub struct NeighbourHandle(Handle);

impl NeighbourHandle {
    pub fn new(handle: Handle) -> Self {
        NeighbourHandle(handle)
    }

    /// List neighbour entries (equivalent to `ip neighbour show`)
    pub fn get(&self) -> NeighbourGetRequest {
        NeighbourGetRequest::new(self.0.clone())
    }

    /// Add a new neighbour entry (equivalent to `ip neighbour add`).
    ///
    /// The [NeighbourMessage] can be built with
    /// [crate::NeighbourMessageBuilder].
    pub fn add(&self, message: NeighbourMessage) -> NeighbourAddRequest {
        NeighbourAddRequest::new(self.0.clone(), message)
    }

    /// Delete a neighbour entry (equivalent to `ip neighbour delete`)
    pub fn del(&self, message: NeighbourMessage) -> NeighbourDelRequest {
        NeighbourDelRequest::new(self.0.clone(), message)
    }
}
