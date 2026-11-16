// SPDX-License-Identifier: MIT

//! This example opens a netlink socket, registers for IPv4 and IPv6 routing
//! changes, listens for said changes and prints the received messages.

use futures_util::stream::StreamExt;
use rtnetlink::{
    new_connection,
    sys::{AsyncSocket, SocketAddr},
};

const RTMGRP_LINK: u32 = 1;
const RTMGRP_IPV4_IFADDR: u32 = 16;
const RTMGRP_IPV4_ROUTE: u32 = 64;
const RTMGRP_IPV6_IFADDR: u32 = 256;
const RTMGRP_IPV6_ROUTE: u32 = 1024;

#[tokio::main]
async fn main() -> Result<(), String> {
    // Open the netlink socket
    let (connection, _, mut messages) =
        new_connection().map_err(|e| format!("{e}"))?;

    // These flags specify what kinds of broadcast messages we want to listen
    // for.
    let mgroup_flags = RTMGRP_LINK
        | RTMGRP_IPV4_IFADDR
        | RTMGRP_IPV4_ROUTE
        | RTMGRP_IPV6_IFADDR
        | RTMGRP_IPV6_ROUTE;

    // A netlink socket address is created with said flags.
    let addr = SocketAddr::new(0, mgroup_flags);
    // Said address is bound so new conenctions and thus new message broadcasts
    // can be received.
    connection
        .socket_ref()
        .socket_ref()
        .bind(&addr)
        .expect("failed to bind");
    tokio::spawn(connection);

    while let Some((message, _)) = messages.next().await {
        let payload = message.payload;
        println!("Route change message - {payload:?}");
    }
    Ok(())
}
