// SPDX-License-Identifier: MIT

use futures::stream::TryStreamExt;
use netlink_packet_route::link::IpVtapMode;
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
        "l2" => IpVtapMode::L2,
        "l3" => IpVtapMode::L3,
        "l3s" => IpVtapMode::L3S,
        _ => {
            usage();
            return Ok(());
        }
    };

    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);

    create_ipvtap(handle, link_name.to_string(), mode)
        .await
        .map_err(|e| format!("{e}"))
}

async fn create_ipvtap(
    handle: Handle,
    link_name: String,
    mode: IpVtapMode,
) -> Result<(), Error> {
    let mut links = handle.link().get().match_name(link_name.clone()).execute();
    if let Some(link) = links.try_next().await? {
        let request = handle.link().add().ipvtap(
            "test_ipvtap".into(),
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
    cargo run --example create_ipvtap -- <link_name> <ipvtap_mode>

ipvtap_mode can be one of the following:
    l2: L2 mode
    l3: L3 mode
    l3s: L3S mode

Note that you need to run this program as root. Instead of running cargo as root,
build the example normally:

    cargo build --example create_ipvtap

Then find the binary in the target directory:

    cd target/debug/examples ; sudo ./create_ipvtap <link_name> <ipvtap_mode>"
    );
}
