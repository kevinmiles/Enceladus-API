use crate::{guid, tests::common::*, tests::user_helpers::*};
use serde_json::{json, Value as Json};

const BASE: &str = "/v1/thread";

fn create_thread(client: &Client, _user_id: i32, token: String) -> Json {
    client
        .post(
            Some(token),
            json!({
                "thread_name": guid(),
                "launch_name": guid(),
                "subreddit": guid(),
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
    let (user_id, user_token) = create_user();
    let created_value = create_thread(&client, user_id, user_token);

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
    let (user_id, user_token) = create_user();

    let thread = json!({
        "thread_name": guid(),
        "launch_name": guid(),
        "subreddit": guid(),
        "t0": rand::random::<i64>(),
        "youtube_id": guid()[0..11],
        "spacex__api_id": guid(),
    });

    let mut body = client
        .post(Some(user_token), &thread)
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
            // auto-generated
            "id": null,
            "created_by_user_id": user_id,
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
    client.delete(None, id);
    delete_user(user_id);
}

#[test]
#[should_panic]
fn create_no_auth() {
    let client = Client::new(BASE);
    let thread = json!({
        "thread_name": guid(),
        "launch_name": guid(),
        "subreddit": guid(),
        "t0": rand::random::<i64>(),
        "youtube_id": guid()[0..11],
        "spacex__api_id": guid(),
    });

    client.post(None, &thread).assert_created();
}

#[test]
fn update() {
    let client = Client::new(BASE);

    // setup
    let (user_id, user_token) = create_user();
    let created_value = create_thread(&client, user_id, user_token);
    assert_eq!(created_value["spacex__api_id"].as_str(), None);

    // test
    let data = json!({ "spacex__api_id": guid() });
    let body = client
        .patch(None, &created_value["id"], &data)
        .assert_ok()
        .get_body_object();
    assert_eq!(body["spacex__api_id"], data["spacex__api_id"]);

    // teardown
    client.delete(None, &created_value["id"]);
    delete_user(user_id);
}

#[test]
fn delete() {
    let client = Client::new(BASE);

    // setup
    let (user_id, user_token) = create_user();
    let created_value = create_thread(&client, user_id, user_token);

    // test
    client
        .delete(None, &created_value["id"])
        .assert_no_content();
    delete_user(user_id);
}
