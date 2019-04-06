use super::{append_log, sleep};
use crate::websocket::CONNECTED_CLIENTS;
use std::sync::atomic::Ordering;
use tokio::await;

#[inline]
pub async fn connected_clients() {
    loop {
        await!(sleep(10));

        await!(append_log(format!(
            "WebSocket connections: {}",
            CONNECTED_CLIENTS.load(Ordering::Relaxed),
        )));
    }
}
