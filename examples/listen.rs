// SPDX-License-Identifier: MIT

//! This example opens a netlink socket, registers for IPv4 and IPv6 routing
//! changes, listens for said changes and prints the received messages.

use futures_util::stream::StreamExt;
use rtnetlink::{new_multicast_connection, MulticastGroup};

#[tokio::main]
async fn main() -> Result<(), String> {
    // Open the netlink socket and register for the broadcast messages we want
    // to listen for.
    let (connection, _, mut messages) = new_multicast_connection(&[
        MulticastGroup::Link,
        MulticastGroup::Ipv4Ifaddr,
        MulticastGroup::Ipv4Route,
        MulticastGroup::Ipv6Ifaddr,
        MulticastGroup::Ipv6Route,
    ])
    .map_err(|e| format!("{e}"))?;
    tokio::spawn(connection);

    while let Some((message, _)) = messages.next().await {
        let payload = message.payload;
        println!("Route change message - {payload:?}");
    }
    Ok(())
}
