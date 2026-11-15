// SPDX-License-Identifier: MIT

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

    /// Add an ip address on an interface (equivalent to `ip addr add`).
    ///
    /// The [AddressMessage] can be built with
    /// [crate::AddressMessageBuilder].
    pub fn add(&self, message: AddressMessage) -> AddressAddRequest {
        AddressAddRequest::new(self.0.clone(), message)
    }

    /// Delete the given address
    pub fn del(&self, address: AddressMessage) -> AddressDelRequest {
        AddressDelRequest::new(self.0.clone(), address)
    }
}
