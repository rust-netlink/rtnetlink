// SPDX-License-Identifier: MIT

use std::net::Ipv4Addr;
use std::process::Command;

use futures::stream::TryStreamExt;
use netlink_packet_core::ErrorMessage;
use netlink_packet_route::tc::TcActionType::Stolen;
use netlink_packet_route::tc::TcMirrorActionType::EgressRedir;
use netlink_packet_route::tc::{
    TcAction, TcActionAttribute, TcActionMessage, TcActionMessageAttribute,
    TcActionMirrorOption, TcActionNatOption, TcActionOption, TcActionType,
    TcMirror, TcNat, TcNatFlags,
};
use netlink_packet_route::{
    tc::{TcAttribute, TcMessage},
    AddressFamily,
};
use tokio::runtime::Runtime;

use crate::{
    new_connection, Error::NetlinkError, TrafficActionGetRequest,
    TrafficActionKind, TrafficActionNewRequest,
};

static TEST_DUMMY_NIC: &str = "netlink-test";

async fn _get_qdiscs() -> Vec<TcMessage> {
    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);
    let mut qdiscs_iter = handle.qdisc().get().execute();
    let mut qdiscs = Vec::new();
    while let Some(nl_msg) = qdiscs_iter.try_next().await.unwrap() {
        qdiscs.push(nl_msg.clone());
    }
    qdiscs
}

#[test]
fn test_get_qdiscs() {
    let qdiscs = Runtime::new().unwrap().block_on(_get_qdiscs());
    let qdisc_of_loopback_nic = &qdiscs[0];
    assert_eq!(qdisc_of_loopback_nic.header.family, AddressFamily::Unspec);
    assert_eq!(qdisc_of_loopback_nic.header.index, 1);
    assert_eq!(qdisc_of_loopback_nic.header.handle, 0.into());
    assert_eq!(qdisc_of_loopback_nic.header.parent, u32::MAX.into());
    assert_eq!(qdisc_of_loopback_nic.header.info, 2); // refcount
    assert_eq!(
        qdisc_of_loopback_nic.attributes[0],
        TcAttribute::Kind("noqueue".to_string())
    );
    assert_eq!(
        qdisc_of_loopback_nic.attributes[1],
        TcAttribute::HwOffload(0)
    );
}

async fn _get_tclasses(ifindex: i32) -> Vec<TcMessage> {
    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);
    let mut tclasses_iter = handle.traffic_class(ifindex).get().execute();
    let mut tclasses = Vec::new();
    while let Some(nl_msg) = tclasses_iter.try_next().await.unwrap() {
        tclasses.push(nl_msg.clone());
    }
    tclasses
}

// Return 0 for not found
fn _get_test_dummy_interface_index() -> i32 {
    let output = Command::new("ip")
        .args(["-o", "link", "show", TEST_DUMMY_NIC])
        .output()
        .expect("failed to run ip command");
    if !output.status.success() {
        0
    } else {
        let line = std::str::from_utf8(&output.stdout).unwrap();
        line.split(": ").next().unwrap().parse::<i32>().unwrap()
    }
}

fn _add_test_dummy_interface() -> i32 {
    if _get_test_dummy_interface_index() == 0 {
        let output = Command::new("ip")
            .args(["link", "add", TEST_DUMMY_NIC, "type", "dummy"])
            .output()
            .expect("failed to run ip command");
        if !output.status.success() {
            eprintln!(
                "Failed to create dummy interface {TEST_DUMMY_NIC} : {output:?}"
            );
        }
        assert!(output.status.success());
    }

    _get_test_dummy_interface_index()
}

fn _remove_test_dummy_interface() {
    let output = Command::new("ip")
        .args(["link", "del", TEST_DUMMY_NIC])
        .output()
        .expect("failed to run ip command");
    if !output.status.success() {
        eprintln!(
            "Failed to remove dummy interface {TEST_DUMMY_NIC} : {output:?}"
        );
    }
    assert!(output.status.success());
}

fn _add_test_tclass_to_dummy() {
    let output = Command::new("tc")
        .args([
            "qdisc",
            "add",
            "dev",
            TEST_DUMMY_NIC,
            "root",
            "handle",
            "1:",
            "htb",
            "default",
            "6",
        ])
        .output()
        .expect("failed to run tc command");
    if !output.status.success() {
        eprintln!(
            "Failed to add qdisc to dummy interface {TEST_DUMMY_NIC} : {output:?}"
        );
    }
    assert!(output.status.success());
    let output = Command::new("tc")
        .args([
            "class",
            "add",
            "dev",
            TEST_DUMMY_NIC,
            "parent",
            "1:",
            "classid",
            "1:1",
            "htb",
            "rate",
            "10mbit",
            "ceil",
            "10mbit",
        ])
        .output()
        .expect("failed to run tc command");
    if !output.status.success() {
        eprintln!(
            "Failed to add traffic class to dummy interface {TEST_DUMMY_NIC}: {output:?}"
        );
    }
    assert!(output.status.success());
}

fn _add_test_filter_to_dummy() {
    let output = Command::new("tc")
        .args([
            "filter",
            "add",
            "dev",
            TEST_DUMMY_NIC,
            "parent",
            "1:",
            "basic",
            "match",
            "meta(priority eq 6)",
            "classid",
            "1:1",
        ])
        .output()
        .expect("failed to run tc command");
    if !output.status.success() {
        eprintln!("Failed to add trafice filter to lo: {output:?}");
    }
    assert!(output.status.success());
}

fn _remove_test_tclass_from_dummy() {
    Command::new("tc")
        .args([
            "class",
            "del",
            "dev",
            TEST_DUMMY_NIC,
            "parent",
            "1:",
            "classid",
            "1:1",
        ])
        .status()
        .unwrap_or_else(|_| {
            panic!(
                "failed to remove tclass from dummy interface {}",
                TEST_DUMMY_NIC
            )
        });
    Command::new("tc")
        .args(["qdisc", "del", "dev", TEST_DUMMY_NIC, "root"])
        .status()
        .unwrap_or_else(|_| {
            panic!(
                "failed to remove qdisc from dummy interface {}",
                TEST_DUMMY_NIC
            )
        });
}

fn _remove_test_filter_from_dummy() {
    Command::new("tc")
        .args(["filter", "del", "dev", TEST_DUMMY_NIC])
        .status()
        .unwrap_or_else(|_| {
            panic!(
                "failed to remove filter from dummy interface {}",
                TEST_DUMMY_NIC
            )
        });
}

async fn _get_filters(ifindex: i32) -> Vec<TcMessage> {
    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);
    let mut filters_iter = handle.traffic_filter(ifindex).get().execute();
    let mut filters = Vec::new();
    while let Some(nl_msg) = filters_iter.try_next().await.unwrap() {
        filters.push(nl_msg.clone());
    }
    filters
}

async fn _get_chains(ifindex: i32) -> Vec<TcMessage> {
    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);
    let mut chains_iter = handle.traffic_chain(ifindex).get().execute();
    let mut chains = Vec::new();
    // The traffic control chain is only supported by kernel 4.19+,
    // hence we might get error: 95 Operation not supported
    loop {
        match chains_iter.try_next().await {
            Ok(Some(nl_msg)) => {
                chains.push(nl_msg.clone());
            }
            Ok(None) => {
                break;
            }
            Err(NetlinkError(ErrorMessage {
                code, header: _, ..
            })) => {
                assert_eq!(code, std::num::NonZeroI32::new(-95));
                eprintln!(
                    "The chain in traffic control is not supported, \
                     please upgrade your kernel"
                );
            }
            _ => {}
        }
    }
    chains
}

// The `cargo test` by default run all tests in parallel, in stead
// of create random named veth/dummy for test, just place class, filter, and
// chain query test in one test case is much simpler.
#[test]
#[cfg_attr(not(feature = "test_as_root"), ignore)]
fn test_get_traffic_classes_filters_and_chains() {
    let ifindex = _add_test_dummy_interface();
    _add_test_tclass_to_dummy();
    _add_test_filter_to_dummy();
    let tclasses = Runtime::new().unwrap().block_on(_get_tclasses(ifindex));
    let filters = Runtime::new().unwrap().block_on(_get_filters(ifindex));
    let chains = Runtime::new().unwrap().block_on(_get_chains(ifindex));
    _remove_test_filter_from_dummy();
    _remove_test_tclass_from_dummy();
    _remove_test_dummy_interface();
    assert_eq!(tclasses.len(), 1);
    let tclass = &tclasses[0];
    assert_eq!(tclass.header.family, AddressFamily::Unspec);
    assert_eq!(tclass.header.index, ifindex);
    assert_eq!(tclass.header.parent, u32::MAX.into());
    assert_eq!(tclass.attributes[0], TcAttribute::Kind("htb".to_string()));
    assert_eq!(filters.len(), 2);
    assert_eq!(filters[0].header.family, AddressFamily::Unspec);
    assert_eq!(filters[0].header.index, ifindex);
    assert_eq!(filters[0].header.parent, (u16::MAX as u32 + 1).into());
    assert_eq!(
        filters[0].attributes[0],
        TcAttribute::Kind("basic".to_string())
    );
    assert_eq!(filters[1].header.family, AddressFamily::Unspec);
    assert_eq!(filters[1].header.index, ifindex);
    assert_eq!(filters[1].header.parent, (u16::MAX as u32 + 1).into());
    assert_eq!(
        filters[1].attributes[0],
        TcAttribute::Kind("basic".to_string())
    );
    assert!(chains.len() <= 1);
    if chains.len() == 1 {
        assert_eq!(chains[0].header.family, AddressFamily::Unspec);
        assert_eq!(chains[0].header.index, ifindex);
        assert_eq!(chains[0].header.parent, (u16::MAX as u32 + 1).into());
        assert_eq!(chains[0].attributes[0], TcAttribute::Chain(0),);
    }
}

async fn _get_actions(kind: TrafficActionKind) -> Vec<TcActionMessage> {
    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);
    let req: TrafficActionGetRequest = handle.traffic_action().get().kind(kind);
    let mut actions_iter = req.execute();
    let mut actions = Vec::new();
    while let Some(nl_msg) = actions_iter.try_next().await.unwrap() {
        actions.push(nl_msg);
    }
    actions
}

fn _flush_test_mirred_action() {
    let output = Command::new("tc")
        .args(["actions", "flush", "action", "mirred"])
        .output()
        .expect("failed to run tc command");
    if !output.status.success() {
        eprintln!("Failed to flush mirred actions: {output:?}");
    }
    assert!(output.status.success());
}

fn _flush_test_nat_action() {
    let output = Command::new("tc")
        .args(["actions", "flush", "action", "nat"])
        .output()
        .expect("failed to run tc command");
    if !output.status.success() {
        eprintln!("Failed to flush nat actions: {output:?}");
    }
    assert!(output.status.success());
}

fn _add_test_mirred_action(index: u32) {
    let output = Command::new("tc")
        .args([
            "actions",
            "add",
            "action",
            "mirred",
            "egress",
            "redirect",
            "dev",
            TEST_DUMMY_NIC,
            "index",
            format!("{index}").as_str(),
        ])
        .output()
        .expect("failed to run tc command");
    if !output.status.success() {
        eprintln!("Failed to add test mirred action: {output:?}");
    }
    assert!(output.status.success());
}

fn _add_test_nat_action(index: u32) {
    let output = Command::new("tc")
        .args([
            "actions",
            "add",
            "action",
            "nat",
            "ingress",
            "1.2.3.4/32",
            "5.6.7.8",
            "index",
            format!("{index}").as_str(),
        ])
        .output()
        .expect("failed to run tc command");
    if !output.status.success() {
        eprintln!("Failed to add test mirred action: {output:?}");
    }
    assert!(output.status.success());
}

#[test]
#[cfg_attr(not(feature = "test_as_root"), ignore)]
fn test_get_mirred_actions() {
    async fn _test() {
        _add_test_dummy_interface();
        // NOTE: this is sketchy if tests run concurrently
        _flush_test_mirred_action();
        let index = 1;
        _add_test_mirred_action(index);
        let actions = _get_actions(TrafficActionKind::Mirror).await;
        assert_eq!(actions.len(), 1);
        let action = &actions[0];
        assert_eq!(action.header.family, AddressFamily::Unspec);
        // Find one and only one `RootCount` attribute and assert that its value
        // is 1
        let root_counts: Vec<_> = action
            .attributes
            .iter()
            .filter_map(|attr| match attr {
                TcActionMessageAttribute::RootCount(count) => Some(count),
                _ => None,
            })
            .collect();
        assert_eq!(root_counts.len(), 1);
        assert_eq!(root_counts[0], &1);
        // Find one and only one `Actions` attribute and assert that it has a
        // length of 1
        let action_lists: Vec<_> = action
            .attributes
            .iter()
            .filter_map(|attr| match attr {
                TcActionMessageAttribute::Actions(actions) => Some(actions),
                _ => None,
            })
            .collect();
        assert_eq!(action_lists.len(), 1);
        assert_eq!(action_lists[0].len(), 1);
        let action = &action_lists[0][0];
        assert_eq!(action.tab, 0);
        assert!(action
            .attributes
            .contains(&TcActionAttribute::Kind("mirred".to_string())));
        let Some(options) =
            action.attributes.iter().find_map(|attr| match attr {
                TcActionAttribute::Options(options) => Some(options),
                _ => None,
            })
        else {
            eprintln!("{action:?}");
            panic!("No options attribute found in action");
        };
        // Assert that we find only mirror options.
        let mirror_options: Vec<_> = options
            .iter()
            .map(|option| match option {
                TcActionOption::Mirror(opt) => opt,
                unexpected => {
                    eprintln!("{unexpected:?}");
                    panic!("Unexpected option type found in mirror action");
                }
            })
            .collect();
        assert!(!mirror_options.is_empty());
        let Some(mirror_params) =
            mirror_options.iter().find_map(|opt| match opt {
                TcActionMirrorOption::Parms(params) => Some(params),
                _ => None,
            })
        else {
            eprintln!("{action:?}");
            panic!("No mirror params found in action");
        };
        assert_eq!(
            mirror_params.ifindex,
            _get_test_dummy_interface_index() as u32
        );
        assert_eq!(mirror_params.eaction, EgressRedir);
        assert_eq!(mirror_params.generic.action, Stolen);
        assert_eq!(mirror_params.generic.bindcnt, 0);
        assert_eq!(mirror_params.generic.index, 1);
        assert_eq!(mirror_params.generic.refcnt, 1);
    }
    Runtime::new().unwrap().block_on(_test());
}

/// NOTE: I consider this test to be overly complex in its structure.
/// It seems like we have an API usability problem.
/// I needed to do a fairly large amount of destructuring to get some
/// fairly basic data.
#[test]
#[cfg_attr(not(feature = "test_as_root"), ignore)]
fn test_add_mirror_action() {
    async fn _test() {
        _add_test_dummy_interface();
        _flush_test_mirred_action();
        let (connection, handle, _) = new_connection().unwrap();
        tokio::spawn(connection);
        let index = 99;
        let mut tc_action = TcAction::default();
        tc_action
            .attributes
            .push(TcActionAttribute::Kind("mirred".to_string()));
        let mut mirror_options = TcMirror::default();
        mirror_options.generic.index = index;
        mirror_options.generic.action = Stolen;
        mirror_options.ifindex = _get_test_dummy_interface_index() as u32;
        mirror_options.eaction = EgressRedir;
        tc_action.attributes.push(TcActionAttribute::Options(vec![
            TcActionOption::Mirror(TcActionMirrorOption::Parms(mirror_options)),
        ]));
        let req: TrafficActionNewRequest =
            handle.traffic_action().add().action(tc_action);
        let mut resp = req.execute();
        if let Some(msg) = resp.try_next().await.unwrap() {
            eprintln!("{:?}", msg);
            panic!("Unexpected response message");
        };
        let resp = _get_actions(TrafficActionKind::Mirror).await;
        assert_eq!(resp.len(), 1);
        let mut checked_interior_props = false;
        resp[0].attributes.iter().for_each(|attr| {
            if let TcActionMessageAttribute::Actions(acts) = attr {
                acts.iter().for_each(|act| {
                    act.attributes.iter().for_each(|act_attr| {
                        if let TcActionAttribute::Options(opts) = act_attr {
                            opts.iter().for_each(|opt| {
                                if let TcActionOption::Mirror(mirror) = opt {
                                    match mirror {
                                        TcActionMirrorOption::Tm(_) => {}
                                        TcActionMirrorOption::Parms(parms) => {
                                            assert_eq!(
                                                parms.ifindex,
                                                _get_test_dummy_interface_index(
                                                )
                                                    as u32
                                            );
                                            assert_eq!(
                                                parms.generic.index,
                                                index
                                            );
                                            assert_eq!(parms.generic.refcnt, 1);
                                            assert_eq!(
                                                parms.generic.bindcnt,
                                                0
                                            );
                                            checked_interior_props = true;
                                        }
                                        _ => {}
                                    }
                                }
                            })
                        }
                    })
                })
            }
        });
        assert!(checked_interior_props);
    }
    Runtime::new().unwrap().block_on(_test());
}

#[test]
#[cfg_attr(not(feature = "test_as_root"), ignore)]
fn test_del_mirror_action() {
    async fn _test() {
        _flush_test_mirred_action();
        _add_test_dummy_interface();
        _add_test_mirred_action(99);
        let (connection, handle, _) = new_connection().unwrap();
        tokio::spawn(connection);
        let mirrors = _get_actions(TrafficActionKind::Mirror).await;
        assert_eq!(mirrors.len(), 1);
        let mut tc_action = TcAction::default();
        tc_action
            .attributes
            .push(TcActionAttribute::Kind("mirred".to_string()));
        tc_action.attributes.push(TcActionAttribute::Index(99));
        let req = handle.traffic_action().del().action(tc_action);
        req.execute().await.unwrap();
        let mirrors = _get_actions(TrafficActionKind::Mirror).await;
        assert!(mirrors.is_empty());
    }
    Runtime::new().unwrap().block_on(_test());
}

#[test]
#[cfg_attr(not(feature = "test_as_root"), ignore)]
fn test_get_nat_actions() {
    async fn _test() {
        _add_test_dummy_interface();
        // NOTE: this is sketchy if tests run concurrently
        _flush_test_nat_action();
        let index = 99;
        _add_test_nat_action(index);
        let actions = _get_actions(TrafficActionKind::Nat).await;
        assert_eq!(actions.len(), 1);
        let action = &actions[0];
        assert_eq!(action.header.family, AddressFamily::Unspec);
        // Find one and only one `RootCount` attribute and assert that its value
        // is 1
        let root_counts: Vec<_> = action
            .attributes
            .iter()
            .filter_map(|attr| match attr {
                TcActionMessageAttribute::RootCount(count) => Some(count),
                _ => None,
            })
            .collect();
        assert_eq!(root_counts.len(), 1);
        assert_eq!(root_counts[0], &1);
        // Find one and only one `Actions` attribute and assert that it has a
        // length of 1
        let action_lists: Vec<_> = action
            .attributes
            .iter()
            .filter_map(|attr| match attr {
                TcActionMessageAttribute::Actions(actions) => Some(actions),
                _ => None,
            })
            .collect();
        assert_eq!(action_lists.len(), 1);
        assert_eq!(action_lists[0].len(), 1);
        let action = &action_lists[0][0];
        assert_eq!(action.tab, 0);
        assert!(action
            .attributes
            .contains(&TcActionAttribute::Kind("nat".to_string())));
        let Some(options) =
            action.attributes.iter().find_map(|attr| match attr {
                TcActionAttribute::Options(options) => Some(options),
                _ => None,
            })
        else {
            eprintln!("{action:?}");
            panic!("No options attribute found in action");
        };
        // Assert that we find only nat options.
        let nat_options: Vec<_> = options
            .iter()
            .map(|option| match option {
                TcActionOption::Nat(opt) => opt,
                unexpected => {
                    eprintln!("{unexpected:?}");
                    panic!("Unexpected option type found in nat action");
                }
            })
            .collect();
        assert!(!nat_options.is_empty());
        let Some(nat_params) = nat_options.iter().find_map(|opt| match opt {
            TcActionNatOption::Parms(params) => Some(params),
            _ => None,
        }) else {
            eprintln!("{action:?}");
            panic!("No mirror params found in action");
        };
        assert_eq!(nat_params.generic.index, index);
        assert_eq!(nat_params.generic.action, TcActionType::Ok);
        assert_eq!(nat_params.generic.refcnt, 1);
        assert_eq!(nat_params.generic.bindcnt, 0);
        assert_eq!(nat_params.old_addr, Ipv4Addr::new(1, 2, 3, 4));
        assert_eq!(nat_params.new_addr, Ipv4Addr::new(5, 6, 7, 8));
        assert_eq!(nat_params.mask, Ipv4Addr::new(255, 255, 255, 255));
        assert_eq!(nat_params.flags, TcNatFlags::empty());
    }
    Runtime::new().unwrap().block_on(_test());
}

/// NOTE: I consider this test to be overly complex in its structure.
/// It seems like we have an API usability problem.
/// I needed to do a fairly large amount of destructuring to get some
/// fairly basic data.
#[test]
#[cfg_attr(not(feature = "test_as_root"), ignore)]
fn test_add_nat_action() {
    async fn _test() {
        _add_test_dummy_interface();
        _flush_test_nat_action();
        let (connection, handle, _) = new_connection().unwrap();
        tokio::spawn(connection);
        let index = 99;
        let mut tc_action = TcAction::default();
        tc_action
            .attributes
            .push(TcActionAttribute::Kind("nat".to_string()));
        let mut nat_options = TcNat::default();
        nat_options.generic.index = index;
        nat_options.generic.action = Stolen;
        nat_options.old_addr = Ipv4Addr::new(1, 2, 3, 4);
        nat_options.new_addr = Ipv4Addr::new(5, 6, 7, 8);
        nat_options.mask = Ipv4Addr::new(255, 255, 255, 255);
        tc_action.attributes.push(TcActionAttribute::Options(vec![
            TcActionOption::Nat(TcActionNatOption::Parms(nat_options)),
        ]));
        let req: TrafficActionNewRequest =
            handle.traffic_action().add().action(tc_action);
        let mut resp = req.execute();
        if let Some(msg) = resp.try_next().await.unwrap() {
            eprintln!("{:?}", msg);
            panic!("Unexpected response message");
        };
        let resp = _get_actions(TrafficActionKind::Nat).await;
        assert_eq!(resp.len(), 1);
        let mut checked_interior_props = false;
        resp[0].attributes.iter().for_each(|attr| {
            if let TcActionMessageAttribute::Actions(acts) = attr {
                acts.iter().for_each(|act| {
                    act.attributes.iter().for_each(|act_attr| {
                        if let TcActionAttribute::Options(opts) = act_attr {
                            opts.iter().for_each(|opt| {
                                if let TcActionOption::Nat(
                                    TcActionNatOption::Parms(parms),
                                ) = opt
                                {
                                    assert_eq!(parms.generic.index, index);
                                    assert_eq!(parms.generic.refcnt, 1);
                                    assert_eq!(parms.generic.bindcnt, 0);
                                    assert_eq!(parms.generic.action, Stolen);
                                    checked_interior_props = true;
                                }
                            })
                        }
                    })
                })
            }
        });
        assert!(checked_interior_props);
    }
    Runtime::new().unwrap().block_on(_test());
}

#[test]
#[cfg_attr(not(feature = "test_as_root"), ignore)]
fn test_del_nat_action() {
    async fn _test() {
        _flush_test_nat_action();
        _add_test_dummy_interface();
        _add_test_nat_action(99);
        let (connection, handle, _) = new_connection().unwrap();
        tokio::spawn(connection);
        let nats = _get_actions(TrafficActionKind::Nat).await;
        assert_eq!(nats.len(), 1);
        let mut tc_action = TcAction::default();
        tc_action
            .attributes
            .push(TcActionAttribute::Kind("nat".to_string()));
        tc_action.attributes.push(TcActionAttribute::Index(99));
        let req = handle.traffic_action().del().action(tc_action);
        req.execute().await.unwrap();
        let nats = _get_actions(TrafficActionKind::Nat).await;
        assert!(nats.is_empty());
    }
    Runtime::new().unwrap().block_on(_test());
}
