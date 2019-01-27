use crate::tests::common::*;
use serde_json::{json, Value as Json};

const BASE: &str = "/v1/thread";

fn create_thread(client: &Client) -> Json {
    client
        .post(json!({
            "thread_name": uuid(),
            "launch_name": uuid(),
            "subreddit": uuid(),
        }))
        .assert_created()
        .get_body_object()
}

#[test]
fn get_all() {
    Client::new(BASE)
        .get_all()
        .assert_ok()
        .assert_body_is_array();
}

#[test]
fn get_one() {
    let client = Client::new(BASE);

    // setup
    let created_value = create_thread(&client);

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

    let thread = json!({
        "thread_name": uuid(),
        "launch_name": uuid(),
        "subreddit": uuid(),
        "t0": rand::random::<i64>(),
        "youtube_id": uuid()[0..11],
        "spacex__api_id": uuid(),
    });

    let mut body = client.post(&thread).assert_created().get_body_object();
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
            "thread_name": thread["thread_name"],
            "launch_name": thread["launch_name"],
            "subreddit": thread["subreddit"],
            "t0": thread["t0"],
            "youtube_id": thread["youtube_id"],
            "spacex__api_id": thread["spacex__api_id"],
        })
    );

    // teardown
    client.delete(id);
}

#[test]
fn update() {
    let client = Client::new(BASE);

    // setup
    let created_value = create_thread(&client);
    assert_eq!(created_value["spacex__api_id"].as_str(), None);

    // test
    let data = json!({ "spacex__api_id": uuid() });
    let body = client
        .patch(&created_value["id"], &data)
        .assert_ok()
        .get_body_object();
    assert_eq!(body["spacex__api_id"], data["spacex__api_id"]);

    // teardown
    client.delete(&created_value["id"]);
}

#[test]
fn delete() {
    let client = Client::new(BASE);

    // setup
    let created_value = create_thread(&client);

    // test
    client.delete(&created_value["id"]).assert_no_content();
}
