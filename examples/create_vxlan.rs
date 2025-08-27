// SPDX-License-Identifier: MIT

use std::env;

use futures::stream::TryStreamExt;
use rtnetlink::{new_connection, Error, Handle, LinkVxlan};

#[tokio::main]
async fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
        return Ok(());
    }
    let link_name = &args[1];

    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);

    create_vxlan(handle, link_name.to_string())
        .await
        .map_err(|e| format!("{e}"))
}

async fn create_vxlan(handle: Handle, name: String) -> Result<(), Error> {
    let mut links = handle.link().get().match_name(name.clone()).execute();
    if let Some(link) = links.try_next().await? {
        let message = LinkVxlan::new("vxlan0", 10)
            .dev(link.header.index)
            .up()
            .port(4789)
            .build();

        handle.link().add(message).execute().await?
    } else {
        println!("no link link {name} found");
    }
    Ok(())
}

fn usage() {
    eprintln!(
        "usage:
    cargo run --example create_vxlan -- <link name>

Note that you need to run this program as root. Instead of running cargo as root,
build the example normally:

    cd netlink-ip ; cargo build --example create_vxlan

Then find the binary in the target directory:

    cd ../target/debug/example ; sudo ./create_vxlan <link_name>"
    );
}
