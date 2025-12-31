// SPDX-License-Identifier: MIT

use futures_util::stream::TryStreamExt;
use rtnetlink::{
    new_connection,
    packet_route::{
        link::{BridgeVlanInfoFlags, LinkAttribute, LinkExtentMask},
        AddressFamily,
    },
    Handle, LinkBridge, LinkBridgeVlan, LinkDummy, LinkUnspec,
};

async fn create_bridge_and_get_index(handle: &Handle) -> Result<u32, String> {
    handle
        .link()
        .add(
            LinkBridge::new("my-bridge0")
                .vlan_filtering(true)
                .up()
                .build(),
        )
        .execute()
        .await
        .map_err(|e| format!("{e}"))?;

    let mut bridge_links = handle
        .link()
        .get()
        .match_name("my-bridge0".to_string())
        .execute();
    if let Some(bridge_link) =
        bridge_links.try_next().await.map_err(|e| format!("{e}"))?
    {
        Ok(bridge_link.header.index)
    } else {
        Err("failed to find my-bridge0".into())
    }
}

async fn create_dummy_and_attach_to_bridge(
    handle: &Handle,
    bridge_index: u32,
) -> Result<u32, rtnetlink::Error> {
    handle
        .link()
        .add(LinkDummy::new("my-dummy0").build())
        .execute()
        .await?;

    handle
        .link()
        .set(
            LinkUnspec::new_with_name("my-dummy0")
                .controller(bridge_index)
                .down()
                .build(),
        )
        .execute()
        .await?;

    let mut dummy_links = handle
        .link()
        .get()
        .match_name("my-dummy0".to_string())
        .execute();
    if let Some(dummy_link) = dummy_links.try_next().await? {
        Ok(dummy_link.header.index)
    } else {
        panic!("failed to find my-dummy0")
    }
}

async fn set_bridge_vlan(
    handle: &Handle,
    port_index: u32,
) -> Result<(), rtnetlink::Error> {
    let message = LinkBridgeVlan::new(port_index)
        .vlan(10, BridgeVlanInfoFlags::Pvid)
        .vlan_range_start(20, BridgeVlanInfoFlags::empty())
        .vlan_range_end(30, BridgeVlanInfoFlags::empty())
        .build();

    handle.link().set(message).execute().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);

    let bridge_index = create_bridge_and_get_index(&handle).await?;

    let port_index =
        create_dummy_and_attach_to_bridge(&handle, bridge_index).await?;
    set_bridge_vlan(&handle, port_index).await?;

    let mut dump_link = handle
        .link()
        .get()
        .set_filter_mask(
            AddressFamily::Bridge,
            vec![LinkExtentMask::BrvlanCompressed],
        )
        .execute();
    while let Some(link_msg) = dump_link.try_next().await? {
        // With set_filter_mask(), we cannot use match_name to filter due to
        // linux kernel limitation.
        if !link_msg.attributes.as_slice().iter().any(
            |a| matches!(a, LinkAttribute::IfName(name) if name == "my-dummy0"),
        ) {
            continue;
        }
        for nla in link_msg.attributes {
            if let LinkAttribute::AfSpecBridge(i) = &nla {
                println!("{:?}", i);
                return Ok(());
            }
        }
    }
    Err("failed to find bridge vlan info for my-dummy0".into())
}
