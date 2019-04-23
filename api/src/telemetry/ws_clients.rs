use super::{append_log, sleep, IncludesTimestamp};
use crate::websocket::CONNECTED_CLIENTS;
use std::sync::atomic::Ordering;
use tokio::await;

#[inline]
pub async fn log_ws_clients() {
    loop {
        await!(sleep(10));

        append_log(
            IncludesTimestamp(false),
            format!(
                "WebSocket connections: {}",
                CONNECTED_CLIENTS.load(Ordering::Relaxed),
            ),
        );
    }
}
