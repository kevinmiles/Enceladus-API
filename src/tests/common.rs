use crate::server;
use rocket::local::{Client, LocalResponse};
use serde_json::Value;

pub fn client() -> Client {
    Client::new(server()).expect("valid rocket instance")
}

pub fn parse_json(value: String) -> serde_json::Result<Value> {
    serde_json::from_str(&value)
}

pub fn uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn body(mut res: LocalResponse) -> Value {
    res.body_string().map(parse_json).unwrap().unwrap() as Value
}
