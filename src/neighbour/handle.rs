// SPDX-License-Identifier: MIT

use std::net::IpAddr;

use netlink_packet_route::neighbour::NeighbourMessage;

use crate::{
    Handle, NeighbourAddRequest, NeighbourDelRequest, NeighbourGetRequest,
    NeighbourMessageBuilder,
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

    /// Add a new neighbour entry (equivalent to `ip neighbour add`).
    ///
    /// This builds a [NeighbourMessage] with [NeighbourMessageBuilder].
    pub fn add_neigh(
        &self,
        index: u32,
        destination: IpAddr,
    ) -> NeighbourAddRequest {
        self.add(build_neigh_message(index, destination))
    }

    /// Delete a neighbour entry (equivalent to `ip neighbour delete`)
    pub fn del(&self, message: NeighbourMessage) -> NeighbourDelRequest {
        NeighbourDelRequest::new(self.0.clone(), message)
    }

    /// Delete a neighbour entry (equivalent to `ip neighbour delete`).
    ///
    /// This builds a [NeighbourMessage] with [NeighbourMessageBuilder].
    pub fn del_neigh(
        &self,
        index: u32,
        destination: IpAddr,
    ) -> NeighbourDelRequest {
        self.del(build_neigh_message(index, destination))
    }

    #[cfg(not(target_os = "freebsd"))]
    /// Add a new fdb entry (equivalent to `bridge fdb add`).
    ///
    /// This builds a [NeighbourMessage] with [NeighbourMessageBuilder].
    pub fn add_fdb(&self, index: u32, lla: &[u8]) -> NeighbourAddRequest {
        self.add(build_fdb_message(index, lla))
    }

    #[cfg(not(target_os = "freebsd"))]
    /// Delete a fdb entry (equivalent to `bridge fdb del`).
    ///
    /// This builds a [NeighbourMessage] with [NeighbourMessageBuilder].
    pub fn del_fdb(&self, index: u32, lla: &[u8]) -> NeighbourDelRequest {
        self.del(build_fdb_message(index, lla))
    }
}

fn build_neigh_message(index: u32, destination: IpAddr) -> NeighbourMessage {
    NeighbourMessageBuilder::new()
        .index(index)
        .destination(destination)
        .build()
}

#[cfg(not(target_os = "freebsd"))]
fn build_fdb_message(index: u32, lla: &[u8]) -> NeighbourMessage {
    NeighbourMessageBuilder::new()
        .index(index)
        .link_layer_address(lla)
        .build()
}
