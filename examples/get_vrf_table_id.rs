// SPDX-License-Identifier: MIT

use futures::stream::TryStreamExt;
use netlink_packet_route::link::{InfoData, InfoVrf, LinkAttribute, LinkInfo};
use rtnetlink::{new_connection, Error, Handle};
use std::env;

#[tokio::main]
async fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
        return Ok(());
    }
    let vrf_name = &args[1];

    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);

    get_vrf_table_id(handle.clone(), vrf_name.to_string())
        .await
        .map_err(|e| format!("{e}"))
}

async fn get_vrf_table_id(handle: Handle, name: String) -> Result<(), Error> {
    let mut links = handle.link().get().match_name(name.clone()).execute();
    let msg = if let Some(msg) = links.try_next().await? {
        msg
    } else {
        eprintln!("[get_vrf_table_id] : no link with name {name} found");
        return Ok(());
    };

    // We should have received only one message
    assert!(links.try_next().await?.is_none());

    for nla in msg.attributes.into_iter() {
        if let LinkAttribute::LinkInfo(lnkinfs) = nla {
            for lnkinf in lnkinfs.into_iter() {
                if let LinkInfo::Data(InfoData::Vrf(vrfinfs)) = lnkinf {
                    for vrfinf in vrfinfs.iter() {
                        if let InfoVrf::TableId(table_id) = vrfinf {
                            println!("VRF:{} TABLE_ID:{:?}", name, table_id);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn usage() {
    eprintln!(
        "usage:
    cargo run --example get_vrf_table_id -- <vrf_name>

Note that you need to run this program as root. Instead of running cargo as root,
build the example normally:

    cd rtnetlink ; cargo build --example get_vrf_table_id

Then find the binary in the target directory:

    cd ../target/debug/example ; sudo ./get_vrf_table_id <vrf_name>"
    );
}
