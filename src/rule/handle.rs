// SPDX-License-Identifier: MIT

use netlink_packet_route::rule::RuleMessage;

use crate::{
    Handle, IpVersion, RuleAddRequest, RuleDelRequest, RuleGetRequest,
};

#[derive(Debug, Clone)]
pub struct RuleHandle(Handle);

impl RuleHandle {
    pub fn new(handle: Handle) -> Self {
        RuleHandle(handle)
    }

    /// Retrieve the list of route rule entries (equivalent to `ip rule show`)
    pub fn get(&self, ip_version: IpVersion) -> RuleGetRequest {
        RuleGetRequest::new(self.0.clone(), ip_version)
    }

    /// Add a route rule entry (equivalent to `ip rule add`).
    ///
    /// The [RuleMessage] can be built with [crate::RuleMessageBuilder].
    pub fn add(&self, message: RuleMessage) -> RuleAddRequest {
        RuleAddRequest::new(self.0.clone(), message)
    }

    /// Delete the given route rule entry (equivalent to `ip rule del`)
    pub fn del(&self, rule: RuleMessage) -> RuleDelRequest {
        RuleDelRequest::new(self.0.clone(), rule)
    }
}
