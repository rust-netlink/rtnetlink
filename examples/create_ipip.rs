// SPDX-License-Identifier: MIT

//! Create an ipip tunnel interface.
//!
//! Equivalent to:
//!   ip link add name ipip0 type ipip local 192.168.1.1 remote 10.0.0.1 ttl 64
//!   ip link set ipip0 up

use std::net::Ipv4Addr;

use rtnetlink::{new_connection, LinkIpIp};

#[tokio::main]
async fn main() -> Result<(), String> {
    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);

    handle
        .link()
        .add(
            LinkIpIp::new("ipip0")
                .local(Ipv4Addr::new(192, 168, 1, 1))
                .remote(Ipv4Addr::new(10, 0, 0, 1))
                .ttl(64)
                .pmtudisc(true)
                .up()
                .build(),
        )
        .execute()
        .await
        .map_err(|e| format!("{e}"))
}
