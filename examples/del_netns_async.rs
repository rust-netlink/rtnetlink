// SPDX-License-Identifier: MIT

#[cfg(not(target_os = "freebsd"))]
use rtnetlink::NetworkNamespace;
use std::env;

#[cfg(target_os = "freebsd")]
fn main() -> () {}

#[cfg(not(target_os = "freebsd"))]
#[async_std::main]
async fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
        return Ok(());
    }
    let ns_name = &args[1];

    NetworkNamespace::del(ns_name.to_string())
        .await
        .map_err(|e| format!("{e}"))
}

fn usage() {
    eprintln!(
        "usage:
    cargo run --example del_netns -- <ns_name>

Note that you need to run this program as root. Instead of running cargo as root,
build the example normally:

    cd netlink-ip ; cargo build --example del_netns

Then find the binary in the target directory:

    cd ../target/debug/example ; sudo ./del_netns <ns_name>"
    );
}
