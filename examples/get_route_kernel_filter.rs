// SPDX-License-Identifier: MIT

use std::net::Ipv4Addr;

use futures_util::stream::TryStreamExt;
use rtnetlink::{
    new_connection,
    packet_route::route::{RouteProtocol, RouteScope, RouteType},
    sys::AsyncSocket,
    RouteMessageBuilder,
};

/// Dump IPv4 routes on table 254 only
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (connection, handle, _) = new_connection().unwrap();

    connection
        .socket_ref()
        .socket_ref()
        .set_netlink_get_strict_chk(true)?;

    tokio::spawn(connection);

    println!("dumping routes for IPv4 in table 254");
    let route = RouteMessageBuilder::<Ipv4Addr>::new()
        .table_id(254)
        .protocol(RouteProtocol::Unspec)
        .scope(RouteScope::Universe)
        .kind(RouteType::Unspec)
        .build();
    let mut routes = handle.route().get(route).execute();
    while let Some(route) = routes.try_next().await? {
        println!("{route:?}");
    }

    Ok(())
}
