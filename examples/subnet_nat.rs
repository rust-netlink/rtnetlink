// SPDX-License-Identifier: MIT
use std::env;
use std::net::Ipv4Addr;

use futures::stream::TryStreamExt;
use netlink_packet_route::tc::{self, nlas::matchall, nlas::nat, Action};
use rtnetlink::{new_connection, Error, Handle};

#[tokio::main]
async fn main() -> Result<(), ()> {
    env_logger::init();

    // Parse the command line
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        usage();
        return Ok(());
    }

    let (old_subnet, prefix_len) = match split_cidr(&args[2]) {
        Ok(addrs) => addrs,
        Err(s) => {
            eprintln!("{}", s);
            return Err(());
        }
    };

    let (new_subnet, _) = match split_cidr(&args[3]) {
        Ok(addrs) => addrs,
        Err(s) => {
            eprintln!("{}", s);
            return Err(());
        }
    };

    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);
    let link_index =
        match get_link_index_by_name(handle.clone(), args[1].clone()).await {
            Ok(i) => i,
            Err(_) => {
                eprintln!("Link: {} not found", args[1]);
                return Err(());
            }
        };

    // Create qdiscs on the interface.
    create_ingress_qdisc(handle.clone(), link_index).await?;
    create_egress_qdisc(&args[1]).await?;

    // Add tc nat action filters

    // First add the egress filter. This is equivalent to the following command:
    // tc filter add dev $devname \
    //     parent 10: protocol ip prio 10 \
    //     matchall action nat egress $old_subnet $new_subnet
    let nat_params = nat::Nla::Parms(
        nat::TcNat::default()
            .set_new_addr(new_subnet)
            .set_old_addr(old_subnet)
            .set_prefix(prefix_len)
            .egress(),
    );

    let mut nat_act = Action::default();
    nat_act.nlas.push(tc::ActNla::Kind(nat::KIND.to_string()));
    nat_act
        .nlas
        .push(tc::ActNla::Options(vec![tc::ActOpt::Nat(nat_params)]));

    let msg = handle
        .traffic_filter(link_index as i32)
        .add()
        .parent(0x10 << 16)
        .priority(10)
        .protocol(0x0008)
        .matchall(vec![matchall::Nla::Act(vec![nat_act])])
        .unwrap();

    if let Err(res) = msg.execute().await {
        eprintln!("{}", res);
        return Err(());
    }

    // Then add the ingress filter. This is equivalent to the following command:
    // tc filter add dev $devname \
    //     parent 10: protocol ip prio 10 \
    //     matchall action nat ingress $new_subnet $old_subnet
    let nat_params = nat::Nla::Parms(
        nat::TcNat::default()
            .set_new_addr(old_subnet)
            .set_old_addr(new_subnet)
            .set_prefix(prefix_len),
    );

    let mut nat_act = Action::default();
    nat_act.nlas.push(tc::ActNla::Kind(nat::KIND.to_string()));
    nat_act
        .nlas
        .push(tc::ActNla::Options(vec![tc::ActOpt::Nat(nat_params)]));

    let msg = handle
        .traffic_filter(link_index as i32)
        .add()
        .parent(0xffff << 16)
        .priority(10)
        .protocol(0x0008)
        .matchall(vec![matchall::Nla::Act(vec![nat_act])])
        .unwrap();

    if let Err(res) = msg.execute().await {
        eprintln!("{}", res);
        return Err(());
    }

    Ok(())
}

// TODO: There is no code in netlink-packet-route for egress qisc types yet.
// This shells out to the `tc` command instead, and should be replaced when
// the appropriate message types are available in netlink-packet-route.
async fn create_egress_qdisc(devname: &str) -> Result<(), ()> {
    match std::process::Command::new("tc")
        .args(&[
            "qdisc", "add", "dev", devname, "root", "handle", "10:", "htb",
        ])
        .output()
    {
        Err(e) => {
            eprintln!("Error creating egress qdisc: {}", e);
            Err(())
        }
        Ok(output) if output.status.success() => Ok(()),
        Ok(_) => {
            eprintln!("Error creating egress qdisc:");
            Err(())
        }
    }
}

async fn create_ingress_qdisc(handle: Handle, index: u32) -> Result<(), ()> {
    if let Err(e) = handle
        .qdisc()
        .add(index as i32)
        .handle(0xffff, 0)
        .ingress()
        .execute()
        .await
    {
        eprintln!("Error creating ingress qdisc: {e}");
        return Err(());
    }

    Ok(())
}

async fn get_link_index_by_name(
    handle: Handle,
    name: String,
) -> Result<u32, Error> {
    let mut links = handle.link().get().match_name(name).execute();
    let link = (links.try_next().await?).expect("Link not found");
    Ok(link.header.index)
}

fn split_cidr(cidr_text: &str) -> Result<(Ipv4Addr, usize), String> {
    let (prefix, len) = cidr_text
        .split_once('/')
        .ok_or(format!("'{}' is not a valid CIDR", cidr_text))?;
    let address: Ipv4Addr = prefix.parse().map_err(|e| {
        format!("'{}' cannot be parsed to an IP address: {}", prefix, e)
    })?;
    let prefix_len: usize = len
        .parse()
        .map_err(|_| format!("'{}' is not a valid prefix length", len))?;

    Ok((address, prefix_len))
}

fn usage() {
    eprintln!(
        "usage:
    cargo run --example subnet_nat -- <devname> <old_subnet> <new_subnet>

This is will have the same effect as:
    tc qdisc add dev $devname root handle 10: htb
    tc qdisc add dev $devname ingress handle ffff

    tc filter add dev $devname parent 10: protocol ip prio 10 matchall action nat egress $old_subnet $new_subnet
    tc filter add dev $devname parent ffff: protocol ip prio 10 matchall action nat ingress $new_subnet $old_subnet

Note that you need to run this program as root. Instead of running cargo as root,
build the example normally:

    cd rtnetlink ; cargo build --example add_tc_qdisc_ingress 

Then find the binary in the target directory:

    cd ../target/debug/example ; sudo ./add_tc_qdisc_ingress <index>"
    );
}
