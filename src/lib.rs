// SPDX-License-Identifier: MIT

//! This crate provides methods to manipulate networking resources (links,
//! addresses, arp tables, route tables) via the netlink protocol.

#![allow(clippy::module_inception)]

pub use netlink_packet_core as packet_core;
pub use netlink_packet_route as packet_route;
pub use netlink_packet_utils as packet_utils;
pub use netlink_proto as proto;
pub use netlink_sys as sys;

mod handle;
pub use crate::handle::*;

#[cfg(not(target_os = "freebsd"))]
mod ns;
#[cfg(not(target_os = "freebsd"))]
pub use crate::ns::*;

mod errors;
pub use crate::errors::*;

mod link;
pub use crate::link::*;

mod addr;
pub use crate::addr::*;

mod route;
pub use crate::route::*;

mod rule;
pub use crate::rule::*;

mod connection;
pub use crate::connection::*;

#[cfg(not(target_os = "freebsd"))]
mod traffic_control;
#[cfg(not(target_os = "freebsd"))]
pub use crate::traffic_control::*;

mod neighbour;
pub use crate::neighbour::*;

pub mod constants;

mod macros;
