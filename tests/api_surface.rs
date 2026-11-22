// SPDX-License-Identifier: MIT

//! Guards the crate's public API surface.
//!
//! Every public type must be nameable through the crate root, so users never
//! have to depend on the layout of private modules.

// The imports below exist only to prove the names are reachable.
#![allow(unused_imports)]

#[test]
fn crate_root_types_are_nameable() {
    #[cfg(any(
        target_os = "linux",
        target_os = "fuchsia",
        target_os = "android"
    ))]
    use rtnetlink::LinkBridgeVlan;
    use rtnetlink::{
        AddrLabelAddRequest, AddrLabelDelRequest, AddrLabelGetRequest,
        AddrLabelHandle, AddrLabelMessageBuilder, AddressAddRequest,
        AddressDelRequest, AddressGetRequest, AddressHandle,
        AddressMessageBuilder, AfstatsRequest, Error, Handle,
        InvalidRouteMessage, IpVersion, LinkAddRequest, LinkAmt,
        LinkAssignNsidHandle, LinkBareudp, LinkBatAdv, LinkBond, LinkBondPort,
        LinkBridge, LinkBridgePort, LinkCan, LinkDelPropRequest,
        LinkDelRequest, LinkDsa, LinkDummy, LinkGeneve, LinkGetNsidHandle,
        LinkGetRequest, LinkGre, LinkGre6, LinkGtp, LinkHandle, LinkHsr,
        LinkIfb, LinkIp6Tnl, LinkIpIp, LinkIpVlan, LinkIpVtap, LinkIpoib,
        LinkMacSec, LinkMacVlan, LinkMacVtap, LinkMessageBuilder,
        LinkNetdevsim, LinkNetkit, LinkNewPropRequest, LinkNlmon, LinkPfcp,
        LinkRmNet, LinkSetRequest, LinkSit, LinkTeam, LinkTun, LinkUnspec,
        LinkVcan, LinkVeth, LinkVirtWifi, LinkVlan, LinkVrf, LinkVti, LinkVti6,
        LinkVxcan, LinkVxlan, LinkWireguard, LinkWwan, LinkXfrm,
        MulticastGroup, NeighbourAddRequest, NeighbourDelRequest,
        NeighbourGetRequest, NeighbourHandle, NeighbourMessageBuilder,
        NetlinkMessageStream, QosMapping, RouteAddRequest, RouteDelRequest,
        RouteGetRequest, RouteHandle, RouteMessageBuilder, RouteNextHopBuilder,
        RuleAddRequest, RuleDelRequest, RuleGetRequest, RuleHandle,
        RuleMessageBuilder,
    };
    #[cfg(not(target_os = "freebsd"))]
    use rtnetlink::{
        NetworkNamespace, QDiscDelRequest, QDiscGetRequest, QDiscHandle,
        QDiscNewRequest, TcMessageBuilder, TrafficChainGetRequest,
        TrafficChainHandle, TrafficClassGetRequest, TrafficClassHandle,
        TrafficFilterDelRequest, TrafficFilterGetRequest, TrafficFilterHandle,
        TrafficFilterNewRequest,
    };
}

#[test]
fn netlink_message_stream_is_send_and_unpin() {
    fn assert_send_unpin<T: Send + Unpin>() {}

    assert_send_unpin::<rtnetlink::NetlinkMessageStream>();
}

fn rust_sources() -> Vec<(std::path::PathBuf, String)> {
    fn collect(dir: &std::path::Path, paths: &mut Vec<std::path::PathBuf>) {
        for entry in std::fs::read_dir(dir).expect("read source directory") {
            let path = entry.expect("read source entry").path();
            if path.is_dir() {
                collect(&path, paths);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                paths.push(path);
            }
        }
    }

    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut paths = Vec::new();
    collect(&src, &mut paths);
    paths.sort();

    paths
        .into_iter()
        .map(|path| {
            let source = std::fs::read_to_string(&path).unwrap_or_else(|e| {
                panic!("failed to read {}: {e}", path.display())
            });
            (path, source)
        })
        .collect()
}

fn has_non_exhaustive_attribute(lines: &[&str], index: usize) -> bool {
    for line in lines[..index].iter().rev() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return false;
        }
        if trimmed == "#[non_exhaustive]" {
            return true;
        }
        if trimmed.starts_with("pub ")
            || trimmed.ends_with('}')
            || trimmed.ends_with(';')
        {
            return false;
        }
    }
    false
}

#[test]
fn public_enums_are_non_exhaustive() {
    const INTENTIONALLY_CLOSED: &[&str] = &["IpVersion"];
    let mut missing = Vec::new();

    for (path, source) in rust_sources() {
        let lines: Vec<&str> = source.lines().collect();
        for (index, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            let Some(rest) = trimmed.strip_prefix("pub enum ") else {
                continue;
            };
            let name = rest
                .split(|c: char| !c.is_alphanumeric() && c != '_')
                .next()
                .unwrap_or("");
            if INTENTIONALLY_CLOSED.contains(&name)
                || has_non_exhaustive_attribute(&lines, index)
            {
                continue;
            }
            missing.push(format!("{}:{} `{name}`", path.display(), index + 1));
        }
    }

    assert!(
        missing.is_empty(),
        "public enums must be `#[non_exhaustive]` or documented as \
         intentionally closed: {missing:#?}"
    );
}

#[test]
fn public_unit_structs_are_non_exhaustive() {
    let mut missing = Vec::new();

    for (path, source) in rust_sources() {
        let lines: Vec<&str> = source.lines().collect();
        for (index, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            let Some(name) = trimmed
                .strip_prefix("pub struct ")
                .and_then(|rest| rest.strip_suffix(';'))
            else {
                continue;
            };
            // Only unit structs (`pub struct Name;`); tuple structs keep
            // their fields private and cannot be built externally anyway.
            if name.is_empty()
                || !name.chars().all(|c| c.is_alphanumeric() || c == '_')
            {
                continue;
            }
            if has_non_exhaustive_attribute(&lines, index) {
                continue;
            }
            missing.push(format!(
                "{}:{} `{trimmed}`",
                path.display(),
                index + 1
            ));
        }
    }

    assert!(
        missing.is_empty(),
        "public unit structs must be `#[non_exhaustive]` so they can gain \
         builder state later: {missing:#?}"
    );
}

#[test]
fn internal_modules_are_not_exposed_as_pub_mod() {
    // Exposing an internal module would give its types a second, unstable
    // path such as `rtnetlink::link::LinkHandle`.
    let public_modules: Vec<&str> = include_str!("../src/lib.rs")
        .lines()
        .map(str::trim)
        .filter_map(|line| line.strip_prefix("pub mod "))
        .map(|module| module.trim_end_matches(';').trim())
        .collect();

    assert!(
        public_modules.is_empty(),
        "internal modules must stay private; re-export public items from the \
         crate root instead, found: {public_modules:?}"
    );
}
