use crate::{guid, tests::common::*};
use serde_json::{json, Value as Json};

const BASE: &str = "/v1/user";

fn create_user_with_body(body: Json) -> (i32, String) {
    let response = Client::new(BASE)
        .post(None, body)
        .assert_created()
        .get_body_object();

    (
        response["id"].as_i64().unwrap() as i32,
        response["token"].as_str().unwrap().to_owned(),
    )
}

pub fn create_user() -> (i32, String) {
    create_user_with_body(json!({
        "reddit_username": guid(),
        "refresh_token": guid(),
    }))
}

pub fn create_global_admin() -> (i32, String) {
    create_user_with_body(json!({
        "reddit_username": guid(),
        "refresh_token": guid(),
        "is_global_admin": true,
    }))
}

pub fn delete_user(id: i32) {
    Client::new(BASE).delete(None, id);
}
