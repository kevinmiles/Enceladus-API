use crate::{guid, tests::helpers::*};
use serde_json::{json, Value as Json};

const BASE: &str = "/v1/preset_event";

fn create_preset_event(client: &mut Client, token: &str) -> Json {
    client
        .with_base(BASE)
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
    Client::new()
        .with_base(BASE)
        .get_all()
        .assert_ok()
        .get_body_array();
}

#[test]
fn get_one() {
    let mut client = Client::new();
    let (user_id, user_token) = user::create_admin(&mut client);

    // setup
    let created_value = create_preset_event(&mut client, &user_token);

    // test
    let body = client
        .with_base(BASE)
        .get(&created_value["id"])
        .assert_ok()
        .get_body_object();
    assert_eq!(created_value, body);

    // teardown
    client.with_base(BASE).delete(None, &body["id"]);
    user::delete(&mut client, user_id);
}

#[test]
fn create() {
    let mut client = Client::new();
    let (user_id, user_token) = user::create_admin(&mut client);

    let event = json!({
        "message": guid(),
        "name": guid(),
    });

    let mut body = client
        .with_base(BASE)
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
    client.with_base(BASE).delete(None, id);
    user::delete(&mut client, user_id);
}

#[test]
fn update() {
    let mut client = Client::new();
    let (user_id, user_token) = user::create_admin(&mut client);

    // setup
    let created_value = create_preset_event(&mut client, &user_token);
    assert_eq!(created_value["holds_clock"].as_bool(), Some(false));

    // test
    let data = json!({ "holds_clock": true });

    let body = client
        .with_base(BASE)
        .patch(Some(user_token), &created_value["id"], &data)
        .assert_ok()
        .get_body_object();
    assert_eq!(body["holds_clock"], data["holds_clock"]);

    // teardown
    client.with_base(BASE).delete(None, &body["id"]);
    user::delete(&mut client, user_id);
}

#[test]
fn delete() {
    let mut client = Client::new();
    let (user_id, user_token) = user::create_admin(&mut client);

    // setup
    let created_value = create_preset_event(&mut client, &user_token);

    // test
    client
        .with_base(BASE)
        .delete(Some(user_token), &created_value["id"])
        .assert_no_content();
    user::delete(&mut client, user_id);
}
