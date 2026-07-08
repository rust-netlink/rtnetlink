// SPDX-License-Identifier: MIT
//
// This example creates a network namespace and assigns it an nsid.
//
// Equivalent to:
//   ip netns add NAME
//   ip netns set NAME NSID
//
// Usage: cargo run --example add_netns -- <name> [nsid]
//
//   sudo cargo run --example add_netns -- test-ns
//   sudo cargo run --example add_netns -- test-ns 42

use std::env;

#[cfg(not(target_os = "freebsd"))]
use rtnetlink::{new_connection, NetworkNamespace};

#[cfg(target_os = "freebsd")]
fn main() -> () {}

#[cfg(not(target_os = "freebsd"))]
#[tokio::main]
async fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <netns-name> [nsid]", args[0]);
        eprintln!(
            "  nsid: integer nsid to assign, or -1 for auto-assign (default)"
        );
        std::process::exit(1);
    }
    let ns_name = &args[1];
    let nsid: i32 = if args.len() > 2 {
        args[2]
            .parse()
            .map_err(|_| format!("Invalid nsid: {}", args[2]))?
    } else {
        -1
    };

    NetworkNamespace::add(ns_name.to_string())
        .await
        .map_err(|e| format!("{e}"))?;

    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);

    handle
        .link()
        .assign_netns_id(ns_name.as_str(), nsid)
        .execute()
        .await
        .map_err(|e| format!("{e}"))?;

    let id = handle
        .link()
        .get_netns_id(ns_name.as_str())
        .execute()
        .await
        .map_err(|e| format!("{e}"))?
        .unwrap_or(-1);

    println!("nsid of \"{ns_name}\": {id}");
    Ok(())
}
