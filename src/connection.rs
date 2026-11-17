// SPDX-License-Identifier: MIT

use std::{
    io,
    pin::Pin,
    task::{Context, Poll},
};

use futures_channel::mpsc::UnboundedReceiver;
use futures_util::stream::{FusedStream, Stream};
use netlink_packet_core::NetlinkMessage;
use netlink_packet_route::RouteNetlinkMessage;
use netlink_proto::{
    sys::{AsyncSocket, SocketAddr},
    Connection,
};

use crate::{Handle, MulticastGroup};

const NETLINK_ROUTE: isize = 0;

type ReceivedMessage = (NetlinkMessage<RouteNetlinkMessage>, SocketAddr);

/// A stream of unsolicited messages received from a netlink connection.
///
/// This type hides the underlying channel implementation so that `rtnetlink`
/// can change it without breaking the public API.
#[derive(Debug)]
pub struct NetlinkMessageStream {
    receiver: UnboundedReceiver<ReceivedMessage>,
}

impl NetlinkMessageStream {
    fn new(receiver: UnboundedReceiver<ReceivedMessage>) -> Self {
        Self { receiver }
    }

    /// Closes the receiving half of the stream.
    ///
    /// Messages that are already queued can still be read until the stream is
    /// drained; any message sent afterwards is dropped.
    pub fn close(&mut self) {
        self.receiver.close();
    }
}

impl Stream for NetlinkMessageStream {
    type Item = ReceivedMessage;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.receiver).poll_next(cx)
    }
}

impl FusedStream for NetlinkMessageStream {
    fn is_terminated(&self) -> bool {
        self.receiver.is_terminated()
    }
}

#[cfg(feature = "tokio_socket")]
#[allow(clippy::type_complexity)]
pub fn new_connection() -> io::Result<(
    Connection<RouteNetlinkMessage>,
    Handle,
    NetlinkMessageStream,
)> {
    new_connection_with_socket()
}

/// Same as [`new_connection()`] but with `NETLINK_GET_STRICT_CHK` enabled,
/// making the kernel reject unsupported attributes of route requests instead
/// of silently ignoring them, like `rtnl_set_strict_dump()` of iproute2.
#[cfg(feature = "tokio_socket")]
#[allow(clippy::type_complexity)]
pub fn new_connection_with_strict_check() -> io::Result<(
    Connection<RouteNetlinkMessage>,
    Handle,
    NetlinkMessageStream,
)> {
    let (conn, handle, messages): (
        Connection<RouteNetlinkMessage>,
        Handle,
        NetlinkMessageStream,
    ) = new_connection_with_socket()?;
    conn.socket_ref()
        .socket_ref()
        .set_netlink_get_strict_chk(true)?;
    Ok((conn, handle, messages))
}

/// Equal to `ip monitor` command
#[cfg(feature = "tokio_socket")]
#[allow(clippy::type_complexity)]
pub fn new_multicast_connection(
    groups: &[MulticastGroup],
) -> io::Result<(
    Connection<RouteNetlinkMessage>,
    Handle,
    NetlinkMessageStream,
)> {
    new_multicast_connection_with_socket(groups)
}

#[allow(clippy::type_complexity)]
pub fn new_connection_with_socket<S>() -> io::Result<(
    Connection<RouteNetlinkMessage, S>,
    Handle,
    NetlinkMessageStream,
)>
where
    S: AsyncSocket,
{
    let (conn, handle, messages) =
        netlink_proto::new_connection_with_socket(NETLINK_ROUTE)
            .map_err(io::Error::other)?;
    Ok((
        conn,
        Handle::new(handle),
        NetlinkMessageStream::new(messages),
    ))
}

/// Equal to `ip monitor` command
#[allow(clippy::type_complexity)]
pub fn new_multicast_connection_with_socket<S>(
    groups: &[MulticastGroup],
) -> io::Result<(
    Connection<RouteNetlinkMessage, S>,
    Handle,
    NetlinkMessageStream,
)>
where
    S: AsyncSocket,
{
    let (conn, handle, messages) = netlink_proto::new_connection_with_socket::<
        RouteNetlinkMessage,
        S,
    >(NETLINK_ROUTE)
    .map_err(io::Error::other)?;
    let mut all_groups: u32 = 0;
    for group in groups.iter().filter(|g| !g.need_via_add_membership()) {
        all_groups |= 1 << (*group as u32 - 1);
    }

    let addr = SocketAddr::new(0, all_groups);
    conn.socket_ref().socket_ref().bind(&addr)?;

    for group in groups.iter().filter(|g| g.need_via_add_membership()) {
        conn.socket_ref()
            .socket_ref()
            .add_membership(*group as u32)?;
    }

    Ok((
        conn,
        Handle::new(handle),
        NetlinkMessageStream::new(messages),
    ))
}

#[allow(clippy::type_complexity)]
pub fn from_socket<S>(
    socket: S,
) -> (
    Connection<RouteNetlinkMessage, S>,
    Handle,
    NetlinkMessageStream,
)
where
    S: AsyncSocket,
{
    let (conn, handle, messages) =
        netlink_proto::from_socket_with_codec(socket);
    (
        conn,
        Handle::new(handle),
        NetlinkMessageStream::new(messages),
    )
}
