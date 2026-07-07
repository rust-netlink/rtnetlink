// SPDX-License-Identifier: MIT

use crate::{
    packet_route::link::{InfoData, InfoIpoib, InfoKind, IpoibMode},
    LinkMessageBuilder,
};

/// Represent IPoIB (IP over InfiniBand) interface.
/// Example code on creating an IPoIB interface
/// ```no_run
/// use rtnetlink::{new_connection, LinkIpoib};
/// use rtnetlink::packet_route::link::IpoibMode;
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let (connection, handle, _) = new_connection().unwrap();
///     tokio::spawn(connection);
///
///     handle
///         .link()
///         .add(
///             LinkIpoib::new("ib0")
///                 .mode(IpoibMode::Connected)
///                 .pkey(0x8001)
///                 .build(),
///         )
///         .execute()
///         .await
///         .map_err(|e| format!("{e}"))
/// }
/// ```
///
/// Please check LinkMessageBuilder::<LinkIpoib> for more detail.
#[derive(Default, Debug)]
pub struct LinkIpoib;

impl LinkIpoib {
    /// Equal to `LinkMessageBuilder::<LinkIpoib>::new()`
    pub fn new(name: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkIpoib>::new(name)
    }
}

impl LinkMessageBuilder<LinkIpoib> {
    /// Create [LinkMessageBuilder] for IPoIB interface type
    pub fn new(name: &str) -> Self {
        LinkMessageBuilder::<LinkIpoib>::new_with_info_kind(InfoKind::Ipoib)
            .name(name.to_string())
    }

    fn append_info_data(self, info: InfoIpoib) -> Self {
        let mut ret = self;
        if let InfoData::Ipoib(infos) = ret
            .info_data
            .get_or_insert_with(|| InfoData::Ipoib(Vec::new()))
        {
            infos.push(info);
        }
        ret
    }

    pub fn pkey(self, pkey: u16) -> Self {
        self.append_info_data(InfoIpoib::Pkey(pkey))
    }

    pub fn mode(self, mode: IpoibMode) -> Self {
        self.append_info_data(InfoIpoib::Mode(mode))
    }

    pub fn umcast(self, umcast: u16) -> Self {
        self.append_info_data(InfoIpoib::UmCast(umcast))
    }
}
