// SPDX-License-Identifier: MIT

use std::net::IpAddr;

use futures::{Stream, TryStreamExt};
use netlink_packet_route::RouteMessage;
use rtnetlink::{new_connection, Error, Handle};

#[tokio::main]
async fn main() {
    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);

    let destinations = [
        "8.8.8.8".parse().unwrap(),
        "127.0.0.8".parse().unwrap(),
        "2001:4860:4860::8888".parse().unwrap(),
        "::1".parse().unwrap(),
    ];
    for dest in destinations {
        println!("getting best route to {}", dest);
        if let Err(e) = dump_route_to(handle.clone(), dest).await {
            eprintln!("{e}");
        }
        println!();
    }
}

async fn dump_route_to(handle: Handle, dest: IpAddr) -> Result<(), Error> {
    let mut routes: Box<
        dyn Stream<Item = Result<RouteMessage, rtnetlink::Error>> + Unpin,
    > = match dest {
        IpAddr::V4(v4) => Box::new(handle.route().get().v4().to(v4).execute()),
        IpAddr::V6(v6) => Box::new(handle.route().get().v6().to(v6).execute()),
    };
    if let Some(route) = routes.try_next().await? {
        println!("{route:?}");
    }
    Ok(())
}
