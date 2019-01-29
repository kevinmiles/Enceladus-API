use crate::{guid, tests::common::*};
use serde_json::{json, Value as Json};

const BASE: &str = "/v1/user";

fn create_user(client: &Client) -> Json {
    client
        .post(json!({
            "reddit_username": guid(),
            "refresh_token": guid(),
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
    let created_value = create_user(&client);

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

    let user = json!({
        "reddit_username": guid(),
        "refresh_token": guid(),
    });

    let mut body = client.post(&user).assert_created().get_body_object();
    assert!(body["id"].is_number(), r#"body["id"] is number"#);
    assert_eq!(body.get("refresh_token"), None);

    // store this so we can perform the teardown
    let id = body["id"].as_i64().unwrap();

    // Remove this, as we don't know what value we should expect.
    // Afterwards, we can ensure that the value is null.
    body["id"].take();
    assert_eq!(
        body,
        json!({
            "id": null,
            "reddit_username": user["reddit_username"],
            "lang": "en",
            "is_global_admin": false,
            "spacex__is_admin": false,
            "spacex__is_mod": false,
            "spacex__is_slack_member": false,
        })
    );

    // teardown
    client.delete(id);
}

#[test]
fn update() {
    let client = Client::new(BASE);

    // setup
    let created_value = create_user(&client);
    assert_eq!(
        created_value["spacex__is_slack_member"].as_bool(),
        Some(false)
    );

    // test
    let data = json!({ "spacex__is_slack_member": true });
    let body = client
        .patch(&created_value["id"], &data)
        .assert_ok()
        .get_body_object();
    assert_eq!(
        body["spacex__is_slack_member"],
        data["spacex__is_slack_member"]
    );

    // teardown
    client.delete(&created_value["id"]);
}

#[test]
fn delete() {
    let client = Client::new(BASE);

    // setup
    let created_value = create_user(&client);

    // test
    client.delete(&created_value["id"]).assert_no_content();
}
