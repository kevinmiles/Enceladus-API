use crate::{guid, tests::common::*, tests::user_helpers::*};
use serde_json::{json, Value as Json};

pub const BASE: &str = "/v1/preset_event";

fn create_preset_event(client: &Client, token: &str) -> Json {
    client
        .post(
            Some(token.to_owned()),
            json!({
                "message": guid(),
                "name": guid(),
            }),
        )
        .assert_created()
        .get_body_object()
}

#[test]
fn get_all() {
    Client::new(BASE).get_all().assert_ok().get_body_array();
}

#[test]
fn get_one() {
    let client = Client::new(BASE);
    let (user_id, user_token) = create_global_admin();

    // setup
    let created_value = create_preset_event(&client, &user_token);

    // test
    let body = client
        .get(&created_value["id"])
        .assert_ok()
        .get_body_object();
    assert_eq!(created_value, body);

    // teardown
    client.delete(None, &body["id"]);
    delete_user(user_id);
}

#[test]
fn create() {
    let client = Client::new(BASE);
    let (user_id, user_token) = create_global_admin();

    let event = json!({
        "message": guid(),
        "name": guid(),
    });

    let mut body = client
        .post(Some(user_token), &event)
        .assert_created()
        .get_body_object();
    assert!(body["id"].is_number(), r#"body["id"] is number"#);

    // store this so we can perform the teardown
    let id = body["id"].as_i64().unwrap();

    // Remove this, as we don't know what value we should expect.
    // Afterwards, we can ensure that the value is null.
    body["id"].take();
    assert_eq!(
        body,
        json!({
            "id": null,
            "holds_clock": false,
            "message": event["message"],
            "name": event["name"],
        })
    );

    // teardown
    client.delete(None, id);
    delete_user(user_id);
}

#[test]
fn update() {
    let client = Client::new(BASE);
    let (user_id, user_token) = create_global_admin();

    // setup
    let created_value = create_preset_event(&client, &user_token);
    assert_eq!(created_value["holds_clock"].as_bool(), Some(false));

    // test
    let data = json!({ "holds_clock": true });

    let body = client
        .patch(Some(user_token), &created_value["id"], &data)
        .assert_ok()
        .get_body_object();
    assert_eq!(body["holds_clock"], data["holds_clock"]);

    // teardown
    client.delete(None, &body["id"]);
    delete_user(user_id);
}

#[test]
fn delete() {
    let client = Client::new(BASE);
    let (user_id, user_token) = create_global_admin();

    // setup
    let created_value = create_preset_event(&client, &user_token);

    // test
    client
        .delete(Some(user_token), &created_value["id"])
        .assert_no_content();
    delete_user(user_id);
}
