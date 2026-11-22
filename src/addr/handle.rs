// SPDX-License-Identifier: MIT

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use netlink_packet_route::address::AddressMessage;

use super::{AddressAddRequest, AddressDelRequest, AddressGetRequest};
use crate::{AddressMessageBuilder, Handle};

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

    /// Add an ip address on an interface (equivalent to `ip addr add`).
    ///
    /// This builds an [AddressMessage] with [AddressMessageBuilder].
    pub fn add_ip(
        &self,
        index: u32,
        address: IpAddr,
        prefix_len: u8,
    ) -> AddressAddRequest {
        self.add(build_address_message(index, address, prefix_len))
    }

    /// Delete the given address
    pub fn del(&self, address: AddressMessage) -> AddressDelRequest {
        AddressDelRequest::new(self.0.clone(), address)
    }

    /// Delete an ip address from an interface (equivalent to `ip addr del`).
    ///
    /// This builds an [AddressMessage] with [AddressMessageBuilder].
    pub fn del_ip(
        &self,
        index: u32,
        address: IpAddr,
        prefix_len: u8,
    ) -> AddressDelRequest {
        self.del(build_address_message(index, address, prefix_len))
    }
}

fn build_address_message(
    index: u32,
    address: IpAddr,
    prefix_len: u8,
) -> AddressMessage {
    match address {
        IpAddr::V4(address) => AddressMessageBuilder::<Ipv4Addr>::new()
            .index(index)
            .address(address, prefix_len)
            .build(),
        IpAddr::V6(address) => AddressMessageBuilder::<Ipv6Addr>::new()
            .index(index)
            .address(address, prefix_len)
            .build(),
    }
}
