// SPDX-License-Identifier: MIT

use std::net::IpAddr;

use netlink_packet_route::address::AddressMessage;

use super::{AddressAddRequest, AddressDelRequest, AddressGetRequest};
use crate::Handle;

#[derive(Debug, Clone)]
pub struct AddressHandle(Handle);

impl AddressHandle {
    pub fn new(handle: Handle) -> Self {
        AddressHandle(handle)
    }

    /// Retrieve the list of ip addresses (equivalent to `ip addr show`)
    pub fn get(&self) -> AddressGetRequest {
        AddressGetRequest::new(self.0.clone())
    }

    /// Add an ip address on an interface (equivalent to `ip addr add`)
    pub fn add(
        &self,
        index: u32,
        address: IpAddr,
        prefix_len: u8,
    ) -> AddressAddRequest {
        AddressAddRequest::new(self.0.clone(), index, address, prefix_len)
    }

    /// Add an address from a pre-built [AddressMessage].
    ///
    /// This is useful for address families that do not have a high-level
    /// helper yet.
    pub fn add_with_message(
        &self,
        message: AddressMessage,
    ) -> AddressAddRequest {
        AddressAddRequest::new_with_message(self.0.clone(), message)
    }

    /// Delete the given address
    pub fn del(&self, address: AddressMessage) -> AddressDelRequest {
        AddressDelRequest::new(self.0.clone(), address)
    }
}
