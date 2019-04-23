use super::{append_log, sleep, IncludesTimestamp};
use rocket_telemetry::Telemetry;
use tokio::await;

#[inline]
pub async fn log_requests() {
    loop {
        await!(sleep(60));

        for request in Telemetry::reset().iter() {
            append_log(IncludesTimestamp(true), request.to_string());
        }
    }
}
