use crate::{guid, tests::helpers::*};
use serde_json::{json, Value as Json};

const BASE: &str = "/v1/thread";

fn create_thread(client: &mut Client, token: &str) -> Json {
    client
        .with_base(BASE)
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
    let (user_id, user_token) = user::create(&mut client);
    let created_value = create_thread(&mut client, &user_token);

    // test
    let body = client
        .with_base(BASE)
        .get(&created_value["id"])
        .assert_ok()
        .get_body_object();
    assert_eq!(created_value, body);

    // teardown
    client
        .with_base(BASE)
        .delete(Some(&user_token), &body["id"]);
    user::delete(&mut client, user_id);
}

#[test]
fn create() {
    let mut client = Client::new();
    let (user_id, user_token) = user::create(&mut client);

    let thread = json!({
        "thread_name": guid(),
        "launch_name": guid(),
        "subreddit": guid(),
        "t0": rand::random::<i64>(),
        "youtube_id": guid()[0..11],
        "spacex__api_id": guid(),
    });

    let mut body = client
        .with_base(BASE)
        .post(Some(&user_token), &thread)
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
    client.with_base(BASE).delete(Some(&user_token), id);
    user::delete(&mut client, user_id);
}

#[test]
#[should_panic]
fn create_no_auth() {
    let mut client = Client::new();
    let thread = json!({
        "thread_name": guid(),
        "launch_name": guid(),
        "subreddit": guid(),
        "t0": rand::random::<i64>(),
        "youtube_id": guid()[0..11],
        "spacex__api_id": guid(),
    });

    client.with_base(BASE).post(None, &thread).assert_created();
}

#[test]
fn update() {
    let mut client = Client::new();

    // setup
    let (user_id, user_token) = user::create(&mut client);
    let created_value = create_thread(&mut client, &user_token);
    assert_eq!(created_value["spacex__api_id"].as_str(), None);

    // test
    let data = json!({ "spacex__api_id": guid() });
    let body = client
        .with_base(BASE)
        .patch(Some(&user_token), &created_value["id"], &data)
        .assert_ok()
        .get_body_object();
    assert_eq!(body["spacex__api_id"], data["spacex__api_id"]);

    // teardown
    client
        .with_base(BASE)
        .delete(Some(&user_token), &created_value["id"]);
    user::delete(&mut client, user_id);
}

#[test]
fn delete() {
    let mut client = Client::new();

    // setup
    let (user_id, user_token) = user::create(&mut client);
    let created_value = create_thread(&mut client, &user_token);

    // test
    client
        .with_base(BASE)
        .delete(Some(&user_token), &created_value["id"])
        .assert_no_content();
    user::delete(&mut client, user_id);
}
