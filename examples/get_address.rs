// SPDX-License-Identifier: MIT

use futures::stream::TryStreamExt;
use rtnetlink::{new_connection, Error, Handle};
use std::env;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Ok(());
    }

    let link_name = &args[1];

    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);

    let link = "lo".to_string();
    println!("dumping address for link \"{link}\"");

    if let Err(e) = dump_addresses(handle.clone(), link).await {
        eprintln!("{e}");
    }

    let label = format!("{link_name}:vip");
    println!("\ndumping address with label \"{label}\"");
    if let Err(e) = dump_addresses_with_label(handle, &label).await {
        eprintln!("{e}");
    }

    Ok(())
}

async fn dump_addresses(handle: Handle, link: String) -> Result<(), Error> {
    let mut links = handle.link().get().match_name(link.clone()).execute();
    if let Some(link) = links.try_next().await? {
        let mut addresses = handle
            .address()
            .get()
            .set_link_index_filter(link.header.index)
            .execute();
        while let Some(msg) = addresses.try_next().await? {
            println!("{msg:?}");
        }
        Ok(())
    } else {
        eprintln!("link {link} not found");
        Ok(())
    }
}

async fn dump_addresses_with_label(
    handle: Handle,
    label: &str,
) -> Result<(), Error> {
    let mut addrs = handle.address().get().set_label_filter(label).execute();

    while let Some(msg) = addrs.try_next().await? {
        println!("{msg:?}");
    }

    Ok(())
}
