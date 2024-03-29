#![feature(const_vec_new, duration_float)]
#![deny(rust_2018_idioms, clippy::all)]
#![warn(clippy::nursery)] // Don't deny, as there may be unknown bugs.

mod request_log;
mod request_timer;

use parking_lot::RwLock;
pub use request_log::RequestLogEntry;
use request_timer::RequestTimer;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{Method, Status},
    Data,
    Request,
    Response,
};
use std::io::Cursor;

pub type RequestLog = Vec<RequestLogEntry>;
pub static REQUESTS: RwLock<RequestLog> = RwLock::new(Vec::new());

#[derive(Default, Debug)]
pub struct Telemetry {}

impl Telemetry {
    /// Reset the telemetry to a fresh state,
    /// returning the existing logs.
    #[inline(always)]
    pub fn reset() -> RequestLog {
        std::mem::replace(&mut REQUESTS.write(), vec![])
    }
}

impl Fairing for Telemetry {
    #[inline]
    fn info(&self) -> Info {
        Info {
            name: "Telemetry",
            kind: Kind::Request | Kind::Response,
        }
    }

    #[inline]
    fn on_request(&self, request: &mut Request<'_>, _: &Data) {
        request.local_cache(RequestTimer::begin);
    }

    #[inline]
    fn on_response(&self, request: &Request<'_>, response: &mut Response<'_>) {
        let start_time = request
            .local_cache(RequestTimer::end)
            .expect("unable to get request start time");
        let duration = start_time.elapsed().expect("error with system time");

        let status = response.status();
        let method = request.method();
        if status == Status::NotFound || method == Method::Options {
            return;
        }

        let body_size = match response.body_bytes() {
            Some(body) => {
                let len = body.len();
                response.set_sized_body(Cursor::new(body));
                len
            }
            None => 0,
        };

        REQUESTS.write().push(RequestLogEntry {
            method,
            uri: request.uri().path().to_string(),
            status,
            body_size,
            duration,
            start_time,
        });
    }
}
