// SPDX-License-Identifier: MIT

use std::{
    env,
    net::{AddrParseError, Ipv6Addr},
};

use ipnetwork::Ipv6Network;
use netlink_packet_route::route::Seg6Mode;
use rtnetlink::{new_connection, Error, Handle, RouteMessageBuilder};

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        usage();
        return Ok(());
    }

    let dst = args[1].parse::<Ipv6Network>().unwrap_or_else(|_| {
        eprintln!("invalid destination address");
        std::process::exit(1);
    });

    let gateway: Ipv6Addr = args[2].parse().unwrap_or_else(|_| {
        eprintln!("invalid gateway");
        std::process::exit(1);
    });

    let mode: Seg6Mode = match args[3].as_str() {
        "inline" => Seg6Mode::Inline,
        "encap" => Seg6Mode::Encap,
        _ => {
            eprintln!("invalid SRv6 mode");
            std::process::exit(1);
        }
    };

    let segments = args[4..]
        .iter()
        .map(|seg| seg.parse::<Ipv6Addr>())
        .collect::<Result<Vec<Ipv6Addr>, AddrParseError>>()
        .unwrap_or_else(|_| {
            eprintln!("invalid SRv6 segments");
            std::process::exit(1);
        });

    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);

    if let Err(e) =
        add_route_seg6(&dst, &gateway, mode, segments, handle.clone()).await
    {
        eprintln!("{e}");
    } else {
        println!("Route has been added");
    }
    Ok(())
}

async fn add_route_seg6(
    dst: &Ipv6Network,
    gateway: &Ipv6Addr,
    mode: Seg6Mode,
    segments: Vec<Ipv6Addr>,
    handle: Handle,
) -> Result<(), Error> {
    let route = RouteMessageBuilder::<Ipv6Addr>::new()
        .destination_prefix(dst.ip(), dst.prefix())
        .gateway(*gateway)
        .output_seg6(mode, segments)
        .build();
    handle.route().add(route).execute().await?;
    Ok(())
}

fn usage() {
    eprintln!(
        "\
usage:
    cargo run --example add_route_seg6 -- <dst> <gateway> <mode> <segments>

Note that you need to run this program as root:

    env CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER='sudo -E' \\
        cargo run --example add_route_seg6 -- <dst> <gateway> \
        <mode> <segments>"
    );
}
