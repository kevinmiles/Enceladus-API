use crate::tests::common::*;
use rocket::{http::Status, local::Client};
use serde_json::{json, Value as Json};

macro_rules! url {
    () => {
        String::from("/v1/preset_event")
    };

    ($id:expr) => {
        format!("{}/{}", url!(), $id)
    };
}

fn create_preset_event(client: &Client) -> Json {
    let event = json!({
        "message": uuid(),
        "name": uuid(),
    })
    .to_string();

    let res = client.post(url!()).body(event).dispatch();
    assert_eq!(res.status(), Status::Created);
    body(res)
}

#[test]
fn get_all() {
    let client = client();

    let res = client.get(url!()).dispatch();
    assert_eq!(res.status(), Status::Ok);

    assert!(body(res).is_array(), "body is array");
}

#[test]
fn get_one() {
    let client = client();

    // setup
    let created_value = create_preset_event(&client);

    // test
    let res = client.get(url!(created_value["id"])).dispatch();
    assert_eq!(res.status(), Status::Ok);

    let body = body(res);
    assert!(body.is_object(), "body is object");
    assert_eq!(created_value, body);

    // teardown
    client.delete(url!(body["id"])).dispatch();
}

#[test]
fn create() {
    let client = client();

    let (message, name) = (uuid(), uuid());
    let event = json!({
        "message": message,
        "name": name,
    })
    .to_string();

    let res = client.post(url!()).body(event).dispatch();
    assert_eq!(res.status(), Status::Created);

    let mut body = body(res);
    assert!(body.is_object(), "body is object");
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
            "message": message,
            "name": name,
        })
    );

    // teardown
    client.delete(url!(id)).dispatch();
}

#[test]
fn update() {
    let client = client();

    // setup
    let created_value = create_preset_event(&client);
    assert_eq!(created_value["holds_clock"].as_bool(), Some(false));

    // test
    let data = json!({ "holds_clock": true }).to_string();

    let res = client
        .patch(url!(created_value["id"]))
        .body(data)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);

    let body = body(res);
    assert!(body.is_object(), "body is object");
    assert_eq!(body["holds_clock"].as_bool(), Some(true));

    // teardown
    client.delete(url!(body["id"])).dispatch();
}

#[test]
fn delete() {
    let client = client();

    // setup
    let created_value = create_preset_event(&client);

    // test
    let res = client.delete(url!(created_value["id"])).dispatch();
    assert_eq!(res.status(), Status::NoContent);
}
