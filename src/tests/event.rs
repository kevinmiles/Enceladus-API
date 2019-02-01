use crate::{guid, tests::common::*};
use serde_json::{json, Value as Json};

const BASE: &str = "/v1/event";

fn create_event(client: &Client) -> Json {
    client
        .post(
            None,
            json!({
                "utc": 1_500_000_000,
                "in_thread_id": 0, // temporary
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

    // setup
    let created_value = create_event(&client);

    // test
    let body = client
        .get(&created_value["id"])
        .assert_ok()
        .get_body_object();
    assert_eq!(created_value, body);

    // teardown
    client.delete(&created_value["id"]);
}

#[test]
fn create() {
    let client = Client::new(BASE);

    let event = json!({
        "message": guid(),
        "terminal_count": guid(),
        "utc": rand::random::<i64>(),
        "in_thread_id": 0, // temporary
    });

    let mut body = client.post(None, &event).assert_created().get_body_object();
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
            "posted": false,
            "message": event["message"],
            "terminal_count": event["terminal_count"],
            "utc": event["utc"],
            "in_thread_id": event["in_thread_id"],
        })
    );

    // teardown
    client.delete(id);
}

#[test]
fn update() {
    let client = Client::new(BASE);

    // setup
    let created_value = create_event(&client);
    assert_eq!(created_value["posted"].as_bool(), Some(false));

    // test
    let data = json!({ "posted": true });
    let body = client
        .patch(&created_value["id"], &data)
        .assert_ok()
        .get_body_object();
    assert_eq!(body["posted"], data["posted"]);

    // teardown
    client.delete(&created_value["id"]);
}

#[test]
fn delete() {
    let client = Client::new(BASE);

    // setup
    let created_value = create_event(&client);

    // test
    client.delete(&created_value["id"]).assert_no_content();
}
