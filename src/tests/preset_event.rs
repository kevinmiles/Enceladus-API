use crate::{guid, tests::common::*};
use serde_json::{json, Value as Json};

pub const BASE: &str = "/v1/preset_event";

fn create_preset_event(client: &Client) -> Json {
    client
        .post(json!({
            "message": guid(),
            "name": guid(),
        }))
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

    // setup
    let created_value = create_preset_event(&client);

    // test
    let body = client
        .get(&created_value["id"])
        .assert_ok()
        .get_body_object();
    assert_eq!(created_value, body);

    // teardown
    client.delete(&body["id"]);
}

#[test]
fn create() {
    let client = Client::new(BASE);

    let event = json!({
        "message": guid(),
        "name": guid(),
    });

    let mut body = client.post(&event).assert_created().get_body_object();
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
    client.delete(id);
}

#[test]
fn update() {
    let client = Client::new(BASE);

    // setup
    let created_value = create_preset_event(&client);
    assert_eq!(created_value["holds_clock"].as_bool(), Some(false));

    // test
    let data = json!({ "holds_clock": true });

    let body = client
        .patch(&created_value["id"], &data)
        .assert_ok()
        .get_body_object();
    assert_eq!(body["holds_clock"], data["holds_clock"]);

    // teardown
    client.delete(&body["id"]);
}

#[test]
fn delete() {
    let client = Client::new(BASE);

    // setup
    let created_value = create_preset_event(&client);

    // test
    client.delete(&created_value["id"]).assert_no_content();
}
