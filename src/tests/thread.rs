use crate::tests::common::*;
use rocket::{http::Status, local::Client};
use serde_json::{json, Value as Json};

macro_rules! url {
    () => {
        String::from("/v1/thread")
    };

    ($id:expr) => {
        format!("{}/{}", url!(), $id)
    };
}

fn create_thread(client: &Client) -> Json {
    let thread = json!({
        "thread_name": uuid(),
        "launch_name": uuid(),
        "subreddit": uuid(),
    })
    .to_string();

    let res = client.post(url!()).body(thread).dispatch();
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
    let created_value = create_thread(&client);

    // test
    let res = client.get(url!(created_value["id"])).dispatch();
    assert_eq!(res.status(), Status::Ok);

    let body = body(res);
    assert!(body.is_object(), "body is object");
    assert_eq!(created_value, body);

    // teardown
    client.delete(url!(body["id"])).dispatch();
}

/// TODO Once authentication is in place,
/// ensure the authenticated user is the one returned in `created_by_user_id`.
#[test]
fn create() {
    let client = client();

    let thread_name = uuid();
    let launch_name = uuid();
    let subreddit = uuid();
    let t0: i32 = rand::random();
    let youtube_id = &uuid()[0..11];
    let api_id = uuid();

    let thread = json!({
        "thread_name": thread_name,
        "launch_name": launch_name,
        "subreddit": subreddit,
        "t0": t0,
        "youtube_id": youtube_id,
        "spacex__api_id": api_id,
    })
    .to_string();

    let res = client.post(url!()).body(thread).dispatch();
    assert_eq!(res.status(), Status::Created);

    let mut body = body(res);
    assert!(body.is_object(), "body is object");
    assert!(body["id"].is_number(), r#"body["id"] is number"#);

    // store this so we can perform the teardown
    let id = body["id"].as_i64().unwrap();

    // Remove this, as we don't know what value we should expect.
    // Afterwards, we can ensure that the value is null.
    body["id"].take();
    body["created_by_user_id"].take();
    assert_eq!(
        body,
        json!({
            // auto-generated
            "id": null,
            "created_by_user_id": null,
            "post_id": null,
            "sections_id": [],
            "events_id": [],

            // user-provided
            "thread_name": thread_name,
            "launch_name": launch_name,
            "subreddit": subreddit,
            "t0": t0,
            "youtube_id": youtube_id,
            "spacex__api_id": api_id,
        })
    );

    // teardown
    client.delete(url!(id)).dispatch();
}

#[test]
fn update() {
    let client = client();

    // setup
    let created_value = create_thread(&client);
    assert_eq!(created_value["spacex__api_id"].as_str(), None);

    // test
    let val = uuid();
    let data = json!({ "spacex__api_id": val }).to_string();

    let res = client
        .patch(url!(created_value["id"]))
        .body(data)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);

    let body = body(res);
    assert!(body.is_object(), "body is object");
    assert_eq!(body["spacex__api_id"].as_str(), Some(&*val));

    // teardown
    client.delete(url!(created_value["id"])).dispatch();
}

#[test]
fn delete() {
    let client = client();

    // setup
    let created_value = create_thread(&client);

    // test
    let res = client.delete(url!(created_value["id"])).dispatch();
    assert_eq!(res.status(), Status::NoContent);
}
