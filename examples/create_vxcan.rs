// SPDX-License-Identifier: MIT

use futures_util::stream::TryStreamExt;
use rtnetlink::{new_connection, Error, Handle, LinkUnspec, LinkVxcan};

#[tokio::main]
async fn main() -> Result<(), String> {
    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);

    let link_name = "vxcan0";
    let peer_name = "vxcan0-peer";

    handle
        .link()
        .add(LinkVxcan::new(link_name, peer_name).build())
        .execute()
        .await
        .map_err(|e| format!("{e}"))?;

    set_link_up(&handle, link_name)
        .await
        .map_err(|e| format!("{e}"))?;

    set_link_up(&handle, peer_name)
        .await
        .map_err(|e| format!("{e}"))?;

    Ok(())
}

async fn set_link_up(handle: &Handle, name: &str) -> Result<(), Error> {
    let mut links = handle.link().get().match_name(name.to_string()).execute();
    if let Some(link) = links.try_next().await? {
        handle
            .link()
            .set(LinkUnspec::new_with_index(link.header.index).up().build())
            .execute()
            .await?
    } else {
        println!("no link {name} found");
    }
    Ok(())
}
