use crate::{guid, tests::helpers::*};
use serde_json::{json, Value as Json};

const BASE: &str = "/v1/user";

fn create_with_body(client: &mut Client, body: Json) -> (i32, String) {
    let response = client
        .with_base(BASE)
        .post(None, body)
        .assert_created()
        .get_body_object();

    (
        response["id"].as_i64().unwrap() as i32,
        response["token"].as_str().unwrap().to_owned(),
    )
}

pub fn create(client: &mut Client) -> (i32, String) {
    create_with_body(
        client,
        json!({
            "reddit_username": guid(),
            "refresh_token": guid(),
        }),
    )
}

pub fn delete(client: &mut Client, id: i32) {
    client.with_base(BASE).delete(None, id);
}
