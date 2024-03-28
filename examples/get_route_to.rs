// SPDX-License-Identifier: MIT

use futures::stream::TryStreamExt;
use rtnetlink::{new_connection, Error, Handle};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[tokio::main]
async fn main() -> Result<(), ()> {
    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);

    println!("dumping specific destination route for IPv4");
    let dest_str = "127.0.0.1";
    let dest: Ipv4Addr = dest_str.parse().expect("Invalid IP address format");
    if let Err(e) = dump_route_to(handle.clone(), IpAddr::V4(dest)).await {
        eprintln!("{e}");
    }
    println!();

    println!("dumping specific destination route for IPv6");
    let dest_str = "::1";
    let dest: Ipv6Addr = dest_str.parse().expect("Invalid IP address format");
    if let Err(e) = dump_route_to(handle.clone(), IpAddr::V6(dest)).await {
        eprintln!("{e}");
    }
    println!();

    Ok(())
}

async fn dump_route_to(
    handle: Handle,
    destination: IpAddr,
) -> Result<(), Error> {
    let mut routes = handle.route().get_to(destination).execute_to();
    while let Some(route) = routes.try_next().await? {
        println!("{route:?}");
    }
    Ok(())
}
