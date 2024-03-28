// SPDX-License-Identifier: MIT

use futures::stream::TryStreamExt;
use rtnetlink::{new_connection, Error, Handle};
use std::env;

#[tokio::main]
async fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        usage();
        return Ok(());
    }
    let link_name = &args[1];
    let mode_str = &args[2];
    let mode = match mode_str.as_str() {
        "l2" => 0u16,
        "l3" => 1u16,
        "l3s" => 2u16,
        _ => {
            usage();
            return Ok(());
        }
    };

    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);

    create_ipvlan(handle, link_name.to_string(), mode)
        .await
        .map_err(|e| format!("{e}"))
}

async fn create_ipvlan(
    handle: Handle,
    link_name: String,
    mode: u16,
) -> Result<(), Error> {
    let mut links = handle.link().get().match_name(link_name.clone()).execute();
    if let Some(link) = links.try_next().await? {
        let request = handle.link().add().ipvlan(
            "test_ipvlan".into(),
            link.header.index,
            mode,
        );
        request.execute().await?
    } else {
        println!("no link {link_name} found");
    }
    Ok(())
}

fn usage() {
    eprintln!(
        "usage:
    cargo run --example create_ipvlan -- <link_name> <ipvlan_mode>

ipvlan_mode can be one of the following:
    l2: L2 mode
    l3: L3 mode
    l3s: L3S mode

Note that you need to run this program as root. Instead of running cargo as root,
build the example normally:

    cargo build --example create_ipvlan

Then find the binary in the target directory:

    cd target/debug/examples ; sudo ./create_ipvlan <link_name> <ipvlan_mode>"
    );
}
