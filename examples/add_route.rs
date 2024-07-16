// SPDX-License-Identifier: MIT

use std::{env, net::Ipv4Addr};

use ipnetwork::Ipv4Network;
use rtnetlink::{new_connection, Error, Handle, RouteMessageBuilder};

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        usage();
        return Ok(());
    }

    let dest: Ipv4Network = args[1].parse().unwrap_or_else(|_| {
        eprintln!("invalid destination");
        std::process::exit(1);
    });
    let gateway: Ipv4Network = args[2].parse().unwrap_or_else(|_| {
        eprintln!("invalid gateway");
        std::process::exit(1);
    });

    let table_id = args[3].parse().unwrap_or_else(|_| {
        eprintln!("invalid table_id");
        std::process::exit(1);
    });

    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);

    if let Err(e) = add_route(&dest, &gateway, table_id, handle.clone()).await {
        eprintln!("{e}");
    } else {
        println!("Route has been added to table {table_id}");
    }
    Ok(())
}

async fn add_route(
    dest: &Ipv4Network,
    gateway: &Ipv4Network,
    table_id: u32,
    handle: Handle,
) -> Result<(), Error> {
    let route = RouteMessageBuilder::<Ipv4Addr>::new()
        .destination_prefix(dest.ip(), dest.prefix())
        .gateway(gateway.ip())
        .table_id(table_id)
        .build();
    handle.route().add(route).execute().await?;
    Ok(())
}

fn usage() {
    eprintln!(
        "usage:
    cargo run --example add_route -- <destination>/<prefix_length> <gateway> <table_id>

Note that you need to run this program as root:

    env CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER='sudo -E' \\
        cargo run --example add_route -- <destination>/<prefix_length> \
        <gateway> <table_id>"
    );
}
