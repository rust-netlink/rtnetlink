// SPDX-License-Identifier: MIT

use std::os::unix::io::RawFd;

use futures::stream::StreamExt;
use netlink_packet_core::{
    NetlinkMessage, NLM_F_ACK, NLM_F_CREATE, NLM_F_EXCL, NLM_F_REQUEST,
};
use netlink_packet_route::{
    link::{LinkAttribute, LinkFlags, LinkMessage},
    RouteNetlinkMessage,
};

use crate::{try_nl, Error, Handle};

pub struct LinkSetRequest {
    handle: Handle,
    message: LinkMessage,
}

impl LinkSetRequest {
    pub(crate) fn new(handle: Handle, index: u32) -> Self {
        let mut message = LinkMessage::default();
        message.header.index = index;
        LinkSetRequest { handle, message }
    }

    /// Execute the request
    pub async fn execute(self) -> Result<(), Error> {
        let LinkSetRequest {
            mut handle,
            message,
        } = self;
        let mut req =
            NetlinkMessage::from(RouteNetlinkMessage::SetLink(message));
        req.header.flags =
            NLM_F_REQUEST | NLM_F_ACK | NLM_F_EXCL | NLM_F_CREATE;

        let mut response = handle.request(req)?;
        while let Some(message) = response.next().await {
            try_nl!(message);
        }
        Ok(())
    }

    /// Return a mutable reference to the request
    pub fn message_mut(&mut self) -> &mut LinkMessage {
        &mut self.message
    }

    /// Attach the link to a bridge (its _master_). This is equivalent to `ip
    /// link set LINK master BRIDGE`. To succeed, both the bridge and the
    /// link that is being attached must be UP.
    ///
    /// To Remove a link from a bridge, set its master to zero.
    /// This is equvalent to `ip link set LINK nomaster`
    #[deprecated(
        since = "0.14.0",
        note = "Please use `LinkSetRequest::controller()` instead"
    )]
    pub fn master(mut self, ctrl_index: u32) -> Self {
        self.message
            .attributes
            .push(LinkAttribute::Controller(ctrl_index));
        self
    }

    /// Attach the link to a bridge (its _controller_). This is equivalent to
    /// `ip link set LINK master BRIDGE`. To succeed, both the bridge and the
    /// link that is being attached must be UP.
    ///
    /// To Remove a link from a bridge, set its master to zero.
    /// This is equvalent to `ip link set LINK nomaster`
    pub fn controller(mut self, ctrl_index: u32) -> Self {
        self.message
            .attributes
            .push(LinkAttribute::Controller(ctrl_index));
        self
    }

    /// Detach the link from its _master_. This is equivalent to `ip link set
    /// LINK nomaster`. To succeed, the link that is being detached must be
    /// UP.
    #[deprecated(
        since = "0.14.0",
        note = "Please use `LinkSetRequest::nocontroller()` instead"
    )]
    pub fn nomaster(mut self) -> Self {
        self.message
            .attributes
            .push(LinkAttribute::Controller(0u32));
        self
    }

    /// Detach the link from its _controller_. This is equivalent to `ip link
    /// set LINK nomaster`. To succeed, the link that is being detached must be
    /// UP.
    pub fn nocontroller(mut self) -> Self {
        self.message
            .attributes
            .push(LinkAttribute::Controller(0u32));
        self
    }

    /// Set the link with the given index up (equivalent to `ip link set dev DEV
    /// up`)
    pub fn up(mut self) -> Self {
        self.message.header.flags |= LinkFlags::Up;
        self.message.header.change_mask |= LinkFlags::Up;
        self
    }

    /// Set the link with the given index down (equivalent to `ip link set dev
    /// DEV down`)
    pub fn down(mut self) -> Self {
        self.message.header.flags.remove(LinkFlags::Up);
        self.message.header.change_mask |= LinkFlags::Up;
        self
    }

    /// Enable or disable promiscious mode of the link with the given index
    /// (equivalent to `ip link set dev DEV promisc on/off`)
    pub fn promiscuous(mut self, enable: bool) -> Self {
        if enable {
            self.message.header.flags |= LinkFlags::Promisc;
        } else {
            self.message.header.flags.remove(LinkFlags::Promisc);
        }
        self.message.header.change_mask |= LinkFlags::Promisc;
        self
    }

    /// Enable or disable the ARP protocol of the link with the given index
    /// (equivalent to `ip link set dev DEV arp on/off`)
    pub fn arp(mut self, enable: bool) -> Self {
        if enable {
            self.message.header.flags.remove(LinkFlags::Noarp);
        } else {
            self.message.header.flags |= LinkFlags::Noarp;
        }
        self.message.header.change_mask |= LinkFlags::Noarp;
        self
    }

    /// Set the name of the link with the given index (equivalent to `ip link
    /// set DEV name NAME`)
    pub fn name(mut self, name: String) -> Self {
        self.message.attributes.push(LinkAttribute::IfName(name));
        self
    }

    /// Set the mtu of the link with the given index (equivalent to `ip link set
    /// DEV mtu MTU`)
    pub fn mtu(mut self, mtu: u32) -> Self {
        self.message.attributes.push(LinkAttribute::Mtu(mtu));
        self
    }

    /// Set the hardware address of the link with the given index (equivalent to
    /// `ip link set DEV address ADDRESS`)
    pub fn address(mut self, address: Vec<u8>) -> Self {
        self.message
            .attributes
            .push(LinkAttribute::Address(address));
        self
    }

    /// Move this network device into the network namespace of the process with
    /// the given `pid`.
    pub fn setns_by_pid(mut self, pid: u32) -> Self {
        self.message.attributes.push(LinkAttribute::NetNsPid(pid));
        self
    }

    /// Move this network device into the network namespace corresponding to the
    /// given file descriptor.
    pub fn setns_by_fd(mut self, fd: RawFd) -> Self {
        self.message.attributes.push(LinkAttribute::NetNsFd(fd));
        self
    }
}
