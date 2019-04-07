mod ws_clients;
mod ws_message;

pub use self::{ws_clients::*, ws_message::*};
use chrono::prelude::*;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use std::time::{Duration, Instant};
use tokio::{await, fs::file::File, prelude::*, timer::Delay};

const LOG_FILE_NAME: &str = "logs.txt";

lazy_static! {
    static ref LOG_FILE: RwLock<File> = RwLock::new(
        std::fs::OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(LOG_FILE_NAME)
            .map(File::from_std)
            .expect("Could not open log file")
    );
}

/// Resolve the future after the provided number of seconds.
#[inline]
async fn sleep(seconds: u64) {
    await!(Delay::new(Instant::now() + Duration::from_secs(seconds)))
        .expect("Error in tokio timer");
}

#[inline]
fn append_log(message: impl Into<Vec<u8>>) {
    // Prevent reallocating as long as the message isn't terribly long.
    let mut bytes = Vec::with_capacity(512);

    // Current time in UTC.
    bytes.append(&mut Utc::now().format("%Y%m%dT%H%M%SZ ").to_string().into());

    // The message provided by the caller.
    bytes.append(&mut message.into());

    // A newline for sanity.
    bytes.push(b'\n');

    // Write to the log file using tokio's `AsyncWrite` trait.
    LOG_FILE
        .write()
        .poll_write(&bytes)
        .expect("Error writing to file");
}

#[inline]
pub fn spawn() {
    tokio::run_async(
        async {
            tokio::spawn_async(log_ws_clients());
        },
    );
}
