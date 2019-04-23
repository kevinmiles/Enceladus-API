use chrono::{offset::Utc, DateTime};
use rocket::http::{Method, Status};
use std::{
    fmt::{self, Display},
    time::{Duration, SystemTime},
};

#[derive(Debug)]
pub struct RequestLogEntry {
    pub method:     Method,
    pub uri:        String,
    pub status:     Status,
    pub body_size:  usize,
    pub duration:   Duration,
    pub start_time: SystemTime,
}

impl Display for RequestLogEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{timestamp}] {duration:.1}ms ({bytes}B) {status} {method} {uri}",
            timestamp = DateTime::<Utc>::from(self.start_time).format("%Y%m%dT%H%M%SZ"),
            // This lets us get the fractional value as well,
            // without having to cast a u128 to a float and then divide.
            duration = self.duration.div_duration(Duration::from_millis(1)),
            bytes = self.body_size,
            status = self.status.code,
            method = self.method,
            uri = self.uri,
        )
    }
}

#[cfg(test)]
#[test]
fn formatting() {
    let entry = RequestLogEntry {
        method:     Method::Get,
        uri:        "/example/path".into(),
        status:     Status::Ok,
        body_size:  4_092,
        duration:   Duration::from_micros(32_768),
        start_time: SystemTime::UNIX_EPOCH,
    };

    assert_eq!(
        "[19700101T000000Z] 32.8ms (4092B) 200 GET /example/path",
        entry.to_string(),
    );
}
