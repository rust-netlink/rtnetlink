// SPDX-License-Identifier: MIT

//! To find the default route for a given ip address family (i.e., IPv4 or
//! IPv6),
//!
//! 1. pick a routing table (in Linux this is almost always table `254` unless
//!    you are using VRFs or something)
//! 1. find all routes in the routing table with
//!    `.header.destination_prefix_length == 0` (that is, routes which are
//!    either `0.0.0.0/0` for IPv4 or `::/0` for IPv6)
//! 2. take the route with the _lowest_ priority among those.

use std::collections::BTreeMap;

use futures::stream::TryStreamExt;
use netlink_packet_route::route::{RouteAttribute, RouteMessage, RouteType};

use rtnetlink::{new_connection, IpVersion};

type RouteTableId = u8;
type RoutePriority = Option<u32>;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);

    let mut default_routes: BTreeMap<
        RouteTableId,
        BTreeMap<RoutePriority, RouteMessage>,
    > = BTreeMap::new();

    // Change to `IpVersion::V6` and this will work for the IPv6 routing tables
    let mut all_routes = handle.clone().route().get(IpVersion::V4).execute();

    while let Ok(Some(route)) = all_routes.try_next().await {
        if route.header.destination_prefix_length != 0 {
            continue;
        }
        if route.header.kind != RouteType::Unicast {
            continue;
        }
        let prio = route.attributes.iter().find_map(|attr| match attr {
            RouteAttribute::Priority(prio) => Some(*prio),
            _ => None,
        });
        if let Some(prio_map) = default_routes.get_mut(&route.header.table) {
            prio_map.insert(prio, route);
        } else {
            let mut prio_map = BTreeMap::new();
            let table_id = route.header.table;
            prio_map.insert(prio, route);
            default_routes.insert(table_id, prio_map);
        }
    }

    for (table, prio_map) in default_routes.iter() {
        println!("Default routes:");
        println!("Table: {table}");
        for (_, route) in prio_map.iter() {
            println!("\t{route:?}");
        }
    }

    Ok(())
}
