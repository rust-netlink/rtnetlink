// SPDX-License-Identifier: MIT

use netlink_packet_core::{NLM_F_CREATE, NLM_F_EXCL, NLM_F_REPLACE};
use netlink_packet_route::tc::TcMessage;

use super::{
    QDiscDelRequest, QDiscGetRequest, QDiscNewRequest, TcMessageBuilder,
    TrafficChainGetRequest, TrafficClassGetRequest, TrafficFilterDelRequest,
    TrafficFilterGetRequest, TrafficFilterNewRequest,
};
use crate::Handle;

#[derive(Debug, Clone)]
pub struct QDiscHandle(Handle);

impl QDiscHandle {
    pub fn new(handle: Handle) -> Self {
        QDiscHandle(handle)
    }

    /// Retrieve the list of qdisc (equivalent to `tc qdisc show`)
    pub fn get(&self) -> QDiscGetRequest {
        QDiscGetRequest::new(self.0.clone())
    }

    /// Create a new qdisc, don't replace if the object already exists.
    /// ( equivalent to `tc qdisc add dev STRING`)
    pub fn add(&self, index: i32) -> QDiscNewRequest {
        let msg = TcMessageBuilder::with_index(index).build();
        QDiscNewRequest::new(self.0.clone(), msg, NLM_F_EXCL | NLM_F_CREATE)
    }

    /// Change the qdisc, the handle cannot be changed and neither can the
    /// parent. In other words, change cannot move a node.
    /// ( equivalent to `tc qdisc change dev STRING`)
    ///
    /// The [TcMessage] can be built with [TcMessageBuilder].
    pub fn change(&self, message: TcMessage) -> QDiscNewRequest {
        QDiscNewRequest::new(self.0.clone(), message, 0)
    }

    /// Change the qdisc on the given interface index.
    /// ( equivalent to `tc qdisc change dev STRING`)
    pub fn change_iface(&self, index: i32) -> QDiscNewRequest {
        self.change(TcMessageBuilder::with_index(index).build())
    }

    /// Replace existing matching qdisc, create qdisc if it doesn't already
    /// exist. ( equivalent to `tc qdisc replace dev STRING`)
    pub fn replace(&self, index: i32) -> QDiscNewRequest {
        let msg = TcMessageBuilder::with_index(index).build();
        QDiscNewRequest::new(self.0.clone(), msg, NLM_F_CREATE | NLM_F_REPLACE)
    }

    /// Performs a replace where the node must exist already.
    /// ( equivalent to `tc qdisc link dev STRING`)
    pub fn link(&self, index: i32) -> QDiscNewRequest {
        let msg = TcMessageBuilder::with_index(index).build();
        QDiscNewRequest::new(self.0.clone(), msg, NLM_F_REPLACE)
    }

    /// Delete the qdisc ( equivalent to `tc qdisc del dev STRING`)
    ///
    /// The [TcMessage] can be built with [TcMessageBuilder].
    pub fn del(&self, message: TcMessage) -> QDiscDelRequest {
        QDiscDelRequest::new(self.0.clone(), message)
    }

    /// Delete the qdisc on the given interface index.
    /// ( equivalent to `tc qdisc del dev STRING`)
    pub fn del_iface(&self, index: i32) -> QDiscDelRequest {
        self.del(TcMessageBuilder::with_index(index).build())
    }
}

#[derive(Debug, Clone)]
pub struct TrafficClassHandle {
    handle: Handle,
    ifindex: i32,
}

impl TrafficClassHandle {
    pub fn new(handle: Handle, ifindex: i32) -> Self {
        TrafficClassHandle { handle, ifindex }
    }

    /// Retrieve the list of traffic class (equivalent to
    /// `tc class show dev <interface_name>`)
    pub fn get(&self) -> TrafficClassGetRequest {
        TrafficClassGetRequest::new(self.handle.clone(), self.ifindex)
    }
}

#[derive(Debug, Clone)]
pub struct TrafficFilterHandle {
    handle: Handle,
    ifindex: i32,
}

impl TrafficFilterHandle {
    pub fn new(handle: Handle, ifindex: i32) -> Self {
        TrafficFilterHandle { handle, ifindex }
    }

    /// Retrieve the list of filter (equivalent to
    /// `tc filter show dev <iface_name>`)
    pub fn get(&self) -> TrafficFilterGetRequest {
        TrafficFilterGetRequest::new(self.handle.clone(), self.ifindex)
    }

    /// Add a filter to a node, don't replace if the object already exists.
    /// ( equivalent to `tc filter add dev STRING`)
    pub fn add(&self) -> TrafficFilterNewRequest {
        TrafficFilterNewRequest::new(
            self.handle.clone(),
            TcMessageBuilder::with_index(self.ifindex).build(),
            NLM_F_EXCL | NLM_F_CREATE,
        )
    }

    /// Delete a filter from a node, don't replace if the object already exists.
    /// ( equivalent to `tc filter del dev STRING`)
    ///
    /// The [TcMessage] can be built with [TcMessageBuilder].
    pub fn del(&self, message: TcMessage) -> TrafficFilterDelRequest {
        TrafficFilterDelRequest::new(self.handle.clone(), message)
    }

    /// Delete a filter from the interface of this handle.
    /// ( equivalent to `tc filter del dev STRING`)
    pub fn del_iface(&self) -> TrafficFilterDelRequest {
        self.del(TcMessageBuilder::with_index(self.ifindex).build())
    }

    /// Change the filter, the handle cannot be changed and neither can the
    /// parent. In other words, change cannot move a node.
    /// ( equivalent to `tc filter change dev STRING`)
    ///
    /// The [TcMessage] can be built with [TcMessageBuilder].
    pub fn change(&self, message: TcMessage) -> TrafficFilterNewRequest {
        TrafficFilterNewRequest::new(self.handle.clone(), message, 0)
    }

    /// Change the filter on the interface of this handle.
    /// ( equivalent to `tc filter change dev STRING`)
    pub fn change_iface(&self) -> TrafficFilterNewRequest {
        self.change(TcMessageBuilder::with_index(self.ifindex).build())
    }

    /// Replace existing matching filter, create filter if it doesn't already
    /// exist. ( equivalent to `tc filter replace dev STRING`)
    pub fn replace(&self) -> TrafficFilterNewRequest {
        TrafficFilterNewRequest::new(
            self.handle.clone(),
            TcMessageBuilder::with_index(self.ifindex).build(),
            NLM_F_CREATE,
        )
    }
}

#[derive(Debug, Clone)]
pub struct TrafficChainHandle {
    handle: Handle,
    ifindex: i32,
}

impl TrafficChainHandle {
    pub fn new(handle: Handle, ifindex: i32) -> Self {
        TrafficChainHandle { handle, ifindex }
    }

    /// Retrieve the list of chain (equivalent to
    /// `tc chain show dev <iface_name>`)
    pub fn get(&self) -> TrafficChainGetRequest {
        TrafficChainGetRequest::new(self.handle.clone(), self.ifindex)
    }
}
