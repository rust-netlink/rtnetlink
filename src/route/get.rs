// SPDX-License-Identifier: MIT

use std::net::{Ipv4Addr, Ipv6Addr};

use futures::{
    future::{self, Either},
    stream::{StreamExt, TryStream},
    FutureExt,
};

use netlink_packet_core::{NetlinkMessage, NLM_F_DUMP, NLM_F_REQUEST};
use netlink_packet_route::{
    route::Nla, RouteMessage, RtnlMessage, AF_INET, AF_INET6, RTN_UNSPEC,
    RTPROT_UNSPEC, RT_SCOPE_UNIVERSE, RT_TABLE_UNSPEC,
};

use crate::{try_rtnl, Error, Handle};

pub struct RouteGetRequest<T = ()> {
    handle: Handle,
    message: RouteMessage,
    destination_address: Option<T>,
}

/// Internet Protocol (IP) version.
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub enum IpVersion {
    /// IPv4
    V4,
    /// IPv6
    V6,
}

impl IpVersion {
    pub(crate) fn family(self) -> u8 {
        match self {
            IpVersion::V4 => AF_INET as u8,
            IpVersion::V6 => AF_INET6 as u8,
        }
    }
}

impl<T> RouteGetRequest<T> {
    pub(crate) fn new(handle: Handle) -> Self {
        let mut message = RouteMessage::default();

        // As per rtnetlink(7) documentation, setting the following
        // fields to 0 gets us all the routes from all the tables
        //
        // > For RTM_GETROUTE, setting rtm_dst_len and rtm_src_len to 0
        // > means you get all entries for the specified routing table.
        // > For the other fields, except rtm_table and rtm_protocol, 0
        // > is the wildcard.
        message.header.destination_prefix_length = 0;
        message.header.source_prefix_length = 0;
        message.header.scope = RT_SCOPE_UNIVERSE;
        message.header.kind = RTN_UNSPEC;

        // I don't know if these two fields matter
        message.header.table = RT_TABLE_UNSPEC;
        message.header.protocol = RTPROT_UNSPEC;

        RouteGetRequest {
            handle,
            message,
            destination_address: None,
        }
    }

    pub fn v4(mut self) -> RouteGetRequest<Ipv4Addr> {
        self.message.header.address_family = AF_INET as u8;
        RouteGetRequest {
            handle: self.handle,
            message: self.message,
            destination_address: None,
        }
    }

    pub fn v6(mut self) -> RouteGetRequest<Ipv6Addr> {
        self.message.header.address_family = AF_INET6 as u8;
        RouteGetRequest {
            handle: self.handle,
            message: self.message,
            destination_address: None,
        }
    }

    pub fn message_mut(&mut self) -> &mut RouteMessage {
        &mut self.message
    }

    pub fn execute(self) -> impl TryStream<Ok = RouteMessage, Error = Error> {
        let RouteGetRequest {
            mut handle,
            message,
            ..
        } = self;

        let mut req = NetlinkMessage::from(RtnlMessage::GetRoute(message));
        req.header.flags = NLM_F_REQUEST;

        if let None = self.destination_address {
            // We want all the route entries, so we set the DUMP flag
            req.header.flags = req.header.flags | NLM_F_DUMP;
        }

        match handle.request(req) {
            Ok(response) => Either::Left(
                response
                    .map(move |msg| Ok(try_rtnl!(msg, RtnlMessage::NewRoute))),
            ),
            Err(e) => Either::Right(
                future::err::<RouteMessage, Error>(e).into_stream(),
            ),
        }
    }
}

impl RouteGetRequest<Ipv4Addr> {
    pub fn destination_address(
        mut self,
        destination_address: Ipv4Addr,
    ) -> Self {
        self.destination_address = Some(destination_address);
        let octets = Vec::from(destination_address.octets());
        self.message.header.destination_prefix_length =
            (octets.len() * 8) as u8;
        self.message.nlas.push(Nla::Destination(octets));
        self
    }
}

impl RouteGetRequest<Ipv6Addr> {
    pub fn destination_address(
        mut self,
        destination_address: Ipv6Addr,
    ) -> Self {
        self.destination_address = Some(destination_address);
        let octets = Vec::from(destination_address.octets());
        self.message.header.destination_prefix_length =
            (octets.len() * 8) as u8;
        self.message.nlas.push(Nla::Destination(octets));
        self
    }
}
