// SPDX-License-Identifier: MIT
use futures::{
    future::{self, Either},
    stream::{StreamExt, TryStream},
    FutureExt,
};
use netlink_packet_core::{NetlinkMessage, NLM_F_DUMP, NLM_F_REQUEST};
use netlink_packet_route::tc::{
    TcAction, TcActionAttribute, TcActionMessage, TcActionMessageAttribute,
    TcActionMessageFlags, TcActionMessageFlagsWithSelector,
};
use netlink_packet_route::{
    tc::{TcHandle, TcMessage},
    AddressFamily, RouteNetlinkMessage,
};

use crate::{try_rtnl, Error, Handle};

pub struct QDiscGetRequest {
    handle: Handle,
    message: TcMessage,
}

impl QDiscGetRequest {
    pub(crate) fn new(handle: Handle) -> Self {
        QDiscGetRequest {
            handle,
            message: TcMessage::default(),
        }
    }

    /// Execute the request
    pub fn execute(self) -> impl TryStream<Ok = TcMessage, Error = Error> {
        let QDiscGetRequest {
            mut handle,
            message,
        } = self;

        let mut req = NetlinkMessage::from(
            RouteNetlinkMessage::GetQueueDiscipline(message),
        );
        req.header.flags = NLM_F_REQUEST | NLM_F_DUMP;

        match handle.request(req) {
            Ok(response) => Either::Left(response.map(move |msg| {
                Ok(try_rtnl!(msg, RouteNetlinkMessage::NewQueueDiscipline))
            })),
            Err(e) => {
                Either::Right(future::err::<TcMessage, Error>(e).into_stream())
            }
        }
    }

    pub fn index(mut self, index: i32) -> Self {
        self.message.header.index = index;
        self
    }

    /// Get ingress qdisc
    pub fn ingress(mut self) -> Self {
        self.message.header.parent = TcHandle::INGRESS;
        self
    }
}

pub struct TrafficClassGetRequest {
    handle: Handle,
    message: TcMessage,
}

impl TrafficClassGetRequest {
    pub(crate) fn new(handle: Handle, ifindex: i32) -> Self {
        let mut message = TcMessage::default();
        message.header.index = ifindex;
        TrafficClassGetRequest { handle, message }
    }

    /// Execute the request
    pub fn execute(self) -> impl TryStream<Ok = TcMessage, Error = Error> {
        let TrafficClassGetRequest {
            mut handle,
            message,
        } = self;

        let mut req =
            NetlinkMessage::from(RouteNetlinkMessage::GetTrafficClass(message));
        req.header.flags = NLM_F_REQUEST | NLM_F_DUMP;

        match handle.request(req) {
            Ok(response) => Either::Left(response.map(move |msg| {
                Ok(try_rtnl!(msg, RouteNetlinkMessage::NewTrafficClass))
            })),
            Err(e) => {
                Either::Right(future::err::<TcMessage, Error>(e).into_stream())
            }
        }
    }
}

pub struct TrafficFilterGetRequest {
    handle: Handle,
    message: TcMessage,
}

impl TrafficFilterGetRequest {
    pub(crate) fn new(handle: Handle, ifindex: i32) -> Self {
        let mut message = TcMessage::default();
        message.header.index = ifindex;
        TrafficFilterGetRequest { handle, message }
    }

    /// Execute the request
    pub fn execute(self) -> impl TryStream<Ok = TcMessage, Error = Error> {
        let TrafficFilterGetRequest {
            mut handle,
            message,
        } = self;

        let mut req = NetlinkMessage::from(
            RouteNetlinkMessage::GetTrafficFilter(message),
        );
        req.header.flags = NLM_F_REQUEST | NLM_F_DUMP;

        match handle.request(req) {
            Ok(response) => Either::Left(response.map(move |msg| {
                Ok(try_rtnl!(msg, RouteNetlinkMessage::NewTrafficFilter))
            })),
            Err(e) => {
                Either::Right(future::err::<TcMessage, Error>(e).into_stream())
            }
        }
    }

    /// Set parent to root.
    pub fn root(mut self) -> Self {
        self.message.header.parent = TcHandle::ROOT;
        self
    }
}

pub struct TrafficChainGetRequest {
    handle: Handle,
    message: TcMessage,
}

impl TrafficChainGetRequest {
    pub(crate) fn new(handle: Handle, ifindex: i32) -> Self {
        let mut message = TcMessage::default();
        message.header.index = ifindex;
        TrafficChainGetRequest { handle, message }
    }

    /// Execute the request
    pub fn execute(self) -> impl TryStream<Ok = TcMessage, Error = Error> {
        let TrafficChainGetRequest {
            mut handle,
            message,
        } = self;

        let mut req =
            NetlinkMessage::from(RouteNetlinkMessage::GetTrafficChain(message));
        req.header.flags = NLM_F_REQUEST | NLM_F_DUMP;

        match handle.request(req) {
            Ok(response) => Either::Left(response.map(move |msg| {
                Ok(try_rtnl!(msg, RouteNetlinkMessage::NewTrafficChain))
            })),
            Err(e) => {
                Either::Right(future::err::<TcMessage, Error>(e).into_stream())
            }
        }
    }
}

/// Request to retrieve traffic actions from the kernel.
/// Equivalent to
///
/// ```bash
/// tc actions list action $action_type
/// ```
#[derive(Debug, Clone)]
#[must_use]
pub struct TrafficActionGetRequest {
    handle: Handle,
    message: TcActionMessage,
}

/// The kind of traffic action.
///
/// This is a list of known traffic actions.
/// If the kernel returns an unknown action, it will be represented as
/// `Other(String)`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TrafficActionKind {
    /// Used for mirroring and redirecting packets.
    Mirror,
    /// Used for network address translation.
    Nat,
    /// Other action type not yet directly supported by this library.
    Other(String),
}

impl<T: AsRef<str>> From<T> for TrafficActionKind {
    fn from(kind: T) -> Self {
        match kind.as_ref() {
            "mirred" => TrafficActionKind::Mirror,
            "nat" => TrafficActionKind::Nat,
            _ => TrafficActionKind::Other(kind.as_ref().into()),
        }
    }
}

impl From<TrafficActionKind> for String {
    fn from(kind: TrafficActionKind) -> Self {
        match kind {
            TrafficActionKind::Mirror => "mirred".into(),
            TrafficActionKind::Nat => "nat".into(),
            TrafficActionKind::Other(kind) => kind,
        }
    }
}

impl TrafficActionGetRequest {
    pub(crate) fn new(handle: Handle) -> Self {
        let mut message = TcActionMessage::default();
        message.header.family = AddressFamily::Unspec;
        let flags = TcActionMessageAttribute::Flags(
            TcActionMessageFlagsWithSelector::new(
                TcActionMessageFlags::LargeDump,
            ),
        );
        message.attributes.push(flags);
        Self { handle, message }
    }

    /// Specify the kind of the action to retrieve.
    pub fn kind<T: Into<TrafficActionKind>>(mut self, kind: T) -> Self {
        let mut tc_action = TcAction::default();
        tc_action
            .attributes
            .push(TcActionAttribute::Kind(String::from(kind.into())));
        let acts = TcActionMessageAttribute::Actions(vec![tc_action]);
        self.message.attributes.push(acts);
        self
    }

    /// Execute the request
    #[must_use]
    pub fn execute(
        self,
    ) -> impl TryStream<Ok = TcActionMessage, Error = Error> {
        let Self {
            mut handle,
            message,
        } = self;

        let mut req = NetlinkMessage::from(
            RouteNetlinkMessage::GetTrafficAction(message),
        );
        req.header.flags = NLM_F_REQUEST | NLM_F_DUMP;

        match handle.request(req) {
            Ok(response) => Either::Left(response.map(move |msg| {
                Ok(try_rtnl!(msg, RouteNetlinkMessage::GetTrafficAction))
            })),
            Err(e) => Either::Right(
                future::err::<TcActionMessage, Error>(e).into_stream(),
            ),
        }
    }
}
