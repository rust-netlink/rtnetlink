// SPDX-License-Identifier: MIT

use rtnetlink::new_connection;

#[tokio::main]
async fn main() -> Result<(), String> {
    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);
    handle
        .link()
        .add()
        .vrf("my-vrf-1".into(), 666)
        .execute()
        .await
        .map_err(|e| format!("{e}"))
}
