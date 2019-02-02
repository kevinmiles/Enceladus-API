use crate::{guid, tests::helpers::*};
use serde_json::{json, Value as Json};

const BASE: &str = "/v1/section";

fn create_section(client: &mut Client) -> Json {
    client
        .with_base(BASE)
        .post(
            None,
            json!({
                "in_thread_id": 0, // temporary
            }),
        )
        .assert_created()
        .get_body_object()
}

#[test]
fn get_all() {
    Client::new()
        .with_base(BASE)
        .get_all()
        .assert_ok()
        .get_body_array();
}

#[test]
fn get_one() {
    let mut client = Client::new();

    // setup
    let created_value = create_section(&mut client);

    // test
    let body = client
        .with_base(BASE)
        .get(&created_value["id"])
        .assert_ok()
        .get_body_object();
    assert_eq!(created_value, body);

    // teardown
    client.with_base(BASE).delete(None, &created_value["id"]);
}

#[test]
fn create() {
    let mut client = Client::new();

    let section = json!({
        "name": guid(),
        "content": guid(),
        "in_thread_id": 0, // temporary
    });

    let mut body = client
        .with_base(BASE)
        .post(None, &section)
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
            "is_events_section": false,
            "name": section["name"],
            "content": section["content"],
            "lock_held_by_user_id": null,
            "in_thread_id": section["in_thread_id"],
        })
    );

    // teardown
    client.with_base(BASE).delete(None, id);
}

#[test]
fn update() {
    let mut client = Client::new();

    // setup
    let created_value = create_section(&mut client);
    assert_eq!(created_value["name"].as_str(), Some(""));

    // test
    let data = json!({ "name": guid() });
    let body = client
        .with_base(BASE)
        .patch(None, &created_value["id"], &data)
        .assert_ok()
        .get_body_object();
    assert_eq!(body["name"], data["name"]);

    // teardown
    client.with_base(BASE).delete(None, &created_value["id"]);
}

#[test]
fn delete() {
    let mut client = Client::new();

    // setup
    let created_value = create_section(&mut client);

    // test
    client
        .with_base(BASE)
        .delete(None, &created_value["id"])
        .assert_no_content();
}
