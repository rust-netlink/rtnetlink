// SPDX-License-Identifier: MIT

use std::env;

use futures_util::stream::TryStreamExt;
use rtnetlink::{
    new_connection,
    packet_route::link::{IpVtapFlags, IpVtapMode},
    Error, Handle, LinkIpVtap,
};

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

    create_ipvtap(handle, link_name.to_string(), mode, IpVtapFlags::empty())
        .await
        .map_err(|e| format!("{e}"))
}

async fn create_ipvtap(
    handle: Handle,
    link_name: String,
    mode: IpVtapMode,
    flags: IpVtapFlags,
) -> Result<(), Error> {
    let mut parent_links =
        handle.link().get().match_name(link_name.clone()).execute();
    if let Some(parent) = parent_links.try_next().await? {
        let builder =
            LinkIpVtap::new("ipvtap_test", parent.header.index, mode, flags)
                .up();
        let message = builder.build();
        let request = handle.link().add(message);

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
