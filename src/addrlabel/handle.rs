// SPDX-License-Identifier: MIT

use crate::{
    AddrLabelAddRequest, AddrLabelDelRequest, AddrLabelGetRequest, Handle,
    IpVersion,
};

#[derive(Debug, Clone)]
pub struct AddrLabelHandle(Handle);

impl AddrLabelHandle {
    pub fn new(handle: Handle) -> Self {
        AddrLabelHandle(handle)
    }

    /// List address labels (equivalent to `ip addrlabel show`)
    pub fn get(&self, ip_version: IpVersion) -> AddrLabelGetRequest {
        AddrLabelGetRequest::new(self.0.clone(), ip_version)
    }

    /// Add an address label (equivalent to `ip addrlabel add`)
    pub fn add(&self) -> AddrLabelAddRequest {
        AddrLabelAddRequest::new(self.0.clone())
    }

    /// Delete an address label (equivalent to `ip addrlabel del`)
    pub fn del(&self) -> AddrLabelDelRequest {
        AddrLabelDelRequest::new(self.0.clone())
    }
}
