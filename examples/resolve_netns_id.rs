// SPDX-License-Identifier: MIT
//
// This example demonstrates how to resolve a network namespace name to its nsid
// using `LinkHandle::get_netns_id()`. It is equivalent to iproute2's:
//
//   ip link set DEV link-netns NAME
//
// Usage: cargo run --example resolve_netns_id -- <netns-name>
//
// First create a netns to test with:
//   sudo ip netns add test-ns
//   sudo cargo run --example resolve_netns_id -- test-ns

use rtnetlink::new_connection;

#[tokio::main]
async fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <netns-name>", args[0]);
        std::process::exit(1);
    }
    let ns_name = &args[1];

    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);

    let maybe_nsid = handle
        .link()
        .get_netns_id(ns_name.as_str())
        .execute()
        .await
        .map_err(|e| format!("Failed to resolve netns \"{ns_name}\": {e}"))?;

    if let Some(nsid) = maybe_nsid {
        println!("nsid of \"{ns_name}\": {nsid}");
    } else {
        println!("\"{ns_name}\" has no nsid assigned");
    }
    Ok(())
}
