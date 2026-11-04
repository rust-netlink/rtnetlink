// SPDX-License-Identifier: MIT

use std::{
    marker::PhantomData,
    mem::discriminant,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};

use netlink_packet_route::{
    address::{
        AddressAttribute, AddressFlags, AddressHeaderFlags, AddressMessage,
        AddressProtocol, AddressScope, CacheInfo,
    },
    AddressFamily,
};

#[derive(Debug)]
/// Helper struct for building [AddressMessage].
pub struct AddressMessageBuilder<T> {
    message: AddressMessage,
    _phantom: PhantomData<T>,
}

impl<T> AddressMessageBuilder<T> {
    /// Create a new [AddressMessageBuilder] without specifying the address
    /// family.
    fn new_no_address_family() -> Self {
        AddressMessageBuilder {
            message: AddressMessage::default(),
            _phantom: PhantomData,
        }
    }

    /// Sets the interface index.
    pub fn index(mut self, index: u32) -> Self {
        self.message.header.index = index;
        self
    }

    /// Sets the address scope (`ifa_scope`), e.g. `AddressScope::Link`.
    pub fn scope(mut self, scope: AddressScope) -> Self {
        set_scope(&mut self.message, scope);
        self
    }

    /// Sets the address flags like the `nodad`, `optimistic`, `home`,
    /// `mngtmpaddr`, `noprefixroute` and `autojoin` options of
    /// `ip address add`.
    ///
    /// The lower 8 bits are stored in the `ifa_flags` field of the message
    /// header. The full 32-bit value is sent as the `IFA_FLAGS` attribute
    /// when any bit does not fit, like iproute2 does.
    pub fn flags(mut self, flags: AddressFlags) -> Self {
        set_flags(&mut self.message, flags);
        self
    }

    /// Sets the interface label (`IFA_LABEL`).
    pub fn label(mut self, label: impl Into<String>) -> Self {
        set_label(&mut self.message, label.into());
        self
    }

    /// Sets the route priority (`IFA_RT_PRIORITY`), which is the `metric`
    /// option of `ip address add`.
    pub fn priority(mut self, priority: u32) -> Self {
        set_priority(&mut self.message, priority);
        self
    }

    /// Sets the cache information (`IFA_CACHEINFO`).
    pub fn cache_info(mut self, cache_info: CacheInfo) -> Self {
        set_cache_info(&mut self.message, cache_info);
        self
    }

    /// Sets the valid lifetime (`ifa_valid` of `IFA_CACHEINFO`).
    ///
    /// When no [AddressMessageBuilder::cache_info] is set, the preferred
    /// lifetime is set to `u32::MAX` (infinite) like iproute2 does for the
    /// omitted `preferred_lft`.
    pub fn valid_lft(mut self, valid_lft: u32) -> Self {
        set_valid_lft(&mut self.message, valid_lft);
        self
    }

    /// Sets the preferred lifetime (`ifa_preferred` of `IFA_CACHEINFO`).
    ///
    /// When no [AddressMessageBuilder::cache_info] is set, the valid
    /// lifetime is set to `u32::MAX` (infinite) like iproute2 does for the
    /// omitted `valid_lft`.
    pub fn preferred_lft(mut self, preferred_lft: u32) -> Self {
        set_preferred_lft(&mut self.message, preferred_lft);
        self
    }

    /// Sets the address protocol (`IFA_PROTO`).
    pub fn protocol(mut self, protocol: AddressProtocol) -> Self {
        set_protocol(&mut self.message, protocol);
        self
    }

    /// Builds [AddressMessage].
    pub fn build(self) -> AddressMessage {
        self.message
    }
}

impl Default for AddressMessageBuilder<Ipv4Addr> {
    fn default() -> Self {
        Self::new()
    }
}

impl AddressMessageBuilder<Ipv4Addr> {
    /// Create a new [AddressMessageBuilder] for IPv4 addresses.
    pub fn new() -> Self {
        let mut builder = Self::new_no_address_family();
        builder.message.header.family = AddressFamily::Inet;
        builder
    }

    /// Sets the address and prefix length.
    pub fn address(mut self, address: Ipv4Addr, prefix_len: u8) -> Self {
        self.message.header.prefix_len = prefix_len;

        if !address.is_multicast() {
            set_attribute(
                &mut self.message,
                AddressAttribute::Address(address.into()),
            );

            // The IFA_LOCAL address can be set to the same value as
            // IFA_ADDRESS.
            set_attribute(
                &mut self.message,
                AddressAttribute::Local(address.into()),
            );

            // Set the IFA_BROADCAST address as well, unless a broadcast
            // address was explicitly requested with
            // [AddressMessageBuilder::broadcast].
            let has_broadcast =
                self.message.attributes.iter().any(|attribute| {
                    matches!(attribute, AddressAttribute::Broadcast(_))
                });
            if prefix_len < 31 && !has_broadcast {
                let ip_addr = u32::from(address);
                let brd = Ipv4Addr::from(
                    (0xffff_ffff_u32) >> u32::from(prefix_len) | ip_addr,
                );
                self.message
                    .attributes
                    .push(AddressAttribute::Broadcast(brd));
            };
        }

        self
    }

    /// Sets the peer address (`IFA_ADDRESS`) for a point-to-point address.
    pub fn peer(mut self, address: Ipv4Addr) -> Self {
        set_peer(&mut self.message, address.into());
        self
    }

    /// Sets the broadcast address (`IFA_BROADCAST`).
    ///
    /// Only IPv4 addresses support broadcast.
    pub fn broadcast(mut self, address: Ipv4Addr) -> Self {
        set_broadcast(&mut self.message, address);
        self
    }
}

impl Default for AddressMessageBuilder<Ipv6Addr> {
    fn default() -> Self {
        Self::new()
    }
}

impl AddressMessageBuilder<Ipv6Addr> {
    /// Create a new [AddressMessageBuilder] for IPv6 addresses.
    pub fn new() -> Self {
        let mut builder = Self::new_no_address_family();
        builder.message.header.family = AddressFamily::Inet6;
        builder
    }

    /// Sets the address and prefix length.
    pub fn address(mut self, address: Ipv6Addr, prefix_len: u8) -> Self {
        self.message.header.prefix_len = prefix_len;

        if address.is_multicast() {
            set_attribute(
                &mut self.message,
                AddressAttribute::Multicast(address),
            );
        } else {
            set_attribute(
                &mut self.message,
                AddressAttribute::Address(address.into()),
            );

            // The IFA_LOCAL address can be set to the same value as
            // IFA_ADDRESS.
            set_attribute(
                &mut self.message,
                AddressAttribute::Local(address.into()),
            );
        }

        self
    }

    /// Sets the peer address (`IFA_ADDRESS`) for a point-to-point address.
    pub fn peer(mut self, address: Ipv6Addr) -> Self {
        set_peer(&mut self.message, address.into());
        self
    }

    /// Sets the anycast address (`IFA_ANYCAST`).
    ///
    /// Only IPv6 addresses support anycast.
    pub fn anycast(mut self, address: Ipv6Addr) -> Self {
        set_anycast(&mut self.message, address);
        self
    }
}

/// Replace the attribute of the same kind or append it when not found, so
/// calling a builder method twice will not create duplicated attributes.
fn set_attribute(message: &mut AddressMessage, attribute: AddressAttribute) {
    let attribute_kind = discriminant(&attribute);
    if let Some(existing) = message
        .attributes
        .iter_mut()
        .find(|existing| discriminant(*existing) == attribute_kind)
    {
        *existing = attribute;
    } else {
        message.attributes.push(attribute);
    }
}

fn get_cache_info(message: &AddressMessage) -> Option<CacheInfo> {
    message
        .attributes
        .iter()
        .find_map(|attribute| match attribute {
            AddressAttribute::CacheInfo(cache_info) => Some(*cache_info),
            _ => None,
        })
}

pub(crate) fn set_scope(message: &mut AddressMessage, scope: AddressScope) {
    message.header.scope = scope;
}

pub(crate) fn set_flags(message: &mut AddressMessage, flags: AddressFlags) {
    // Only the first byte of the address flags fits into the message header.
    // Like iproute2, the 32-bit `IFA_FLAGS` attribute is used for the flags
    // which cannot be stored in `ifa_flags` of the header.
    if flags.bits() <= u8::MAX as u32 {
        message.header.flags =
            AddressHeaderFlags::from_bits_retain(flags.bits() as u8);
        message.attributes.retain(|attribute| {
            !matches!(attribute, AddressAttribute::Flags(_))
        });
    } else {
        message.header.flags = AddressHeaderFlags::empty();
        set_attribute(message, AddressAttribute::Flags(flags));
    }
}

pub(crate) fn set_label(message: &mut AddressMessage, label: String) {
    set_attribute(message, AddressAttribute::Label(label));
}

pub(crate) fn set_priority(message: &mut AddressMessage, priority: u32) {
    set_attribute(message, AddressAttribute::RoutePriority(priority));
}

pub(crate) fn set_cache_info(
    message: &mut AddressMessage,
    cache_info: CacheInfo,
) {
    set_attribute(message, AddressAttribute::CacheInfo(cache_info));
}

pub(crate) fn set_valid_lft(message: &mut AddressMessage, valid_lft: u32) {
    let mut cache_info = match get_cache_info(message) {
        Some(cache_info) => cache_info,
        None => infinite_cache_info(),
    };
    cache_info.ifa_valid = valid_lft;
    set_cache_info(message, cache_info);
}

pub(crate) fn set_preferred_lft(
    message: &mut AddressMessage,
    preferred_lft: u32,
) {
    let mut cache_info = match get_cache_info(message) {
        Some(cache_info) => cache_info,
        None => infinite_cache_info(),
    };
    cache_info.ifa_preferred = preferred_lft;
    set_cache_info(message, cache_info);
}

fn infinite_cache_info() -> CacheInfo {
    let mut cache_info = CacheInfo::default();
    cache_info.ifa_preferred = u32::MAX;
    cache_info.ifa_valid = u32::MAX;
    cache_info
}

pub(crate) fn set_protocol(
    message: &mut AddressMessage,
    protocol: AddressProtocol,
) {
    set_attribute(message, AddressAttribute::Protocol(protocol));
}

pub(crate) fn set_peer(message: &mut AddressMessage, peer: IpAddr) {
    set_attribute(message, AddressAttribute::Address(peer));
}

pub(crate) fn set_broadcast(message: &mut AddressMessage, broadcast: Ipv4Addr) {
    set_attribute(message, AddressAttribute::Broadcast(broadcast));
}

pub(crate) fn set_anycast(message: &mut AddressMessage, anycast: Ipv6Addr) {
    set_attribute(message, AddressAttribute::Anycast(anycast));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope() {
        let message = AddressMessageBuilder::<Ipv4Addr>::new()
            .scope(AddressScope::Link)
            .build();
        assert_eq!(message.header.scope, AddressScope::Link);
    }

    #[test]
    fn test_flags_fitting_header() {
        let message = AddressMessageBuilder::<Ipv6Addr>::new()
            .flags(AddressFlags::Nodad)
            .build();
        assert_eq!(message.header.flags, AddressHeaderFlags::Nodad);
        assert!(!message
            .attributes
            .iter()
            .any(|attribute| matches!(attribute, AddressAttribute::Flags(_))));
    }

    #[test]
    fn test_flags_using_attribute() {
        let message = AddressMessageBuilder::<Ipv6Addr>::new()
            .flags(AddressFlags::Noprefixroute)
            .build();
        assert_eq!(message.header.flags, AddressHeaderFlags::empty());
        assert!(message
            .attributes
            .contains(&AddressAttribute::Flags(AddressFlags::Noprefixroute)));
    }

    #[test]
    fn test_flags_replace_previous_value() {
        let message = AddressMessageBuilder::<Ipv6Addr>::new()
            .flags(AddressFlags::Nodad)
            .flags(AddressFlags::Noprefixroute)
            .build();
        assert_eq!(message.header.flags, AddressHeaderFlags::empty());
        let flags = message
            .attributes
            .iter()
            .filter(|attribute| matches!(attribute, AddressAttribute::Flags(_)))
            .count();
        assert_eq!(flags, 1);
        assert!(message
            .attributes
            .contains(&AddressAttribute::Flags(AddressFlags::Noprefixroute)));

        let message = AddressMessageBuilder::<Ipv6Addr>::new()
            .flags(AddressFlags::Noprefixroute)
            .flags(AddressFlags::Nodad)
            .build();
        assert_eq!(message.header.flags, AddressHeaderFlags::Nodad);
        assert!(!message
            .attributes
            .iter()
            .any(|attribute| matches!(attribute, AddressAttribute::Flags(_))));
    }

    #[test]
    fn test_ipv4_auto_broadcast() {
        let message = AddressMessageBuilder::<Ipv4Addr>::new()
            .address(Ipv4Addr::new(192, 0, 2, 1), 24)
            .build();
        assert!(message.attributes.contains(&AddressAttribute::Broadcast(
            Ipv4Addr::new(192, 0, 2, 255)
        )));
    }

    #[test]
    fn test_ipv4_explicit_broadcast_is_kept() {
        let message = AddressMessageBuilder::<Ipv4Addr>::new()
            .broadcast(Ipv4Addr::new(192, 0, 2, 254))
            .address(Ipv4Addr::new(192, 0, 2, 1), 24)
            .build();
        let broadcasts = message
            .attributes
            .iter()
            .filter(|attribute| {
                matches!(attribute, AddressAttribute::Broadcast(_))
            })
            .count();
        assert_eq!(broadcasts, 1);
        assert!(message.attributes.contains(&AddressAttribute::Broadcast(
            Ipv4Addr::new(192, 0, 2, 254)
        )));
    }

    #[test]
    fn test_ipv4_peer() {
        let message = AddressMessageBuilder::<Ipv4Addr>::new()
            .address(Ipv4Addr::new(198, 51, 100, 1), 24)
            .peer(Ipv4Addr::new(198, 51, 100, 2))
            .build();
        assert!(message.attributes.contains(&AddressAttribute::Address(
            IpAddr::V4(Ipv4Addr::new(198, 51, 100, 2))
        )));
        assert!(message.attributes.contains(&AddressAttribute::Local(
            IpAddr::V4(Ipv4Addr::new(198, 51, 100, 1))
        )));
    }

    #[test]
    fn test_ipv6_anycast() {
        let anycast = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0x3);
        let message = AddressMessageBuilder::<Ipv6Addr>::new()
            .address(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0x2), 64)
            .anycast(anycast)
            .build();
        assert!(message
            .attributes
            .contains(&AddressAttribute::Anycast(anycast)));
    }

    #[test]
    fn test_lifetimes() {
        let message = AddressMessageBuilder::<Ipv4Addr>::new()
            .valid_lft(1000)
            .preferred_lft(500)
            .build();
        let mut expected = CacheInfo::default();
        expected.ifa_valid = 1000;
        expected.ifa_preferred = 500;
        assert!(message
            .attributes
            .contains(&AddressAttribute::CacheInfo(expected)));
    }

    #[test]
    fn test_single_lifetime_defaults_to_infinite() {
        let message = AddressMessageBuilder::<Ipv4Addr>::new()
            .valid_lft(1000)
            .build();
        let cache_info =
            message
                .attributes
                .iter()
                .find_map(|attribute| match attribute {
                    AddressAttribute::CacheInfo(cache_info) => {
                        Some(*cache_info)
                    }
                    _ => None,
                });
        let cache_info = cache_info.unwrap();
        assert_eq!(cache_info.ifa_valid, 1000);
        assert_eq!(cache_info.ifa_preferred, u32::MAX);
    }

    #[test]
    fn test_label_priority_and_protocol() {
        let message = AddressMessageBuilder::<Ipv4Addr>::new()
            .label("eth0:1")
            .priority(42)
            .protocol(AddressProtocol::Other(7))
            .build();
        assert!(message
            .attributes
            .contains(&AddressAttribute::Label("eth0:1".to_string())));
        assert!(message
            .attributes
            .contains(&AddressAttribute::RoutePriority(42)));
        assert!(message
            .attributes
            .contains(&AddressAttribute::Protocol(AddressProtocol::Other(7))));
    }

    #[test]
    fn test_setters_replace_attributes() {
        let message = AddressMessageBuilder::<Ipv4Addr>::new()
            .label("eth0:1")
            .label("eth0:2")
            .priority(1)
            .priority(2)
            .build();
        let labels = message
            .attributes
            .iter()
            .filter(|attribute| matches!(attribute, AddressAttribute::Label(_)))
            .count();
        let priorities = message
            .attributes
            .iter()
            .filter(|attribute| {
                matches!(attribute, AddressAttribute::RoutePriority(_))
            })
            .count();
        assert_eq!(labels, 1);
        assert_eq!(priorities, 1);
        assert!(message
            .attributes
            .contains(&AddressAttribute::Label("eth0:2".to_string())));
        assert!(message
            .attributes
            .contains(&AddressAttribute::RoutePriority(2)));
    }
}
