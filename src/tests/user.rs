use crate::tests::common::*;
use rocket::{http::Status, local::Client};
use serde_json::{json, Value as Json};

macro_rules! url {
    () => {
        String::from("/v1/user")
    };

    ($id:expr) => {
        format!("{}/{}", url!(), $id)
    };
}

fn create_user(client: &Client) -> Json {
    let user = json!({
        "reddit_username": uuid(),
        "refresh_token": uuid(),
    })
    .to_string();

    let res = client.post(url!()).body(user).dispatch();
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
    let created_value = create_user(&client);

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

    let reddit_username = uuid();
    let user = json!({
        "reddit_username": reddit_username,
        "refresh_token": uuid(),
    })
    .to_string();

    let res = client.post(url!()).body(user).dispatch();
    assert_eq!(res.status(), Status::Created);

    let mut body = body(res);
    assert!(body.is_object(), "body is object");
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
            "reddit_username": reddit_username,
            "lang": "en",
            "is_global_admin": false,
            "spacex__is_admin": false,
            "spacex__is_mod": false,
            "spacex__is_slack_member": false,
        })
    );

    // teardown
    client.delete(url!(id)).dispatch();
}

#[test]
fn update() {
    let client = client();

    // setup
    let created_value = create_user(&client);
    assert_eq!(
        created_value["spacex__is_slack_member"].as_bool(),
        Some(false)
    );

    // test
    let data = json!({ "spacex__is_slack_member": true }).to_string();

    let res = client
        .patch(url!(created_value["id"]))
        .body(data)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);

    let body = body(res);
    assert!(body.is_object(), "body is object");
    assert_eq!(body["spacex__is_slack_member"].as_bool(), Some(true));

    // teardown
    client.delete(url!(body["id"])).dispatch();
}

#[test]
fn delete() {
    let client = client();

    // setup
    let created_value = create_user(&client);

    // test
    let res = client.delete(url!(created_value["id"])).dispatch();
    assert_eq!(res.status(), Status::NoContent);
}
