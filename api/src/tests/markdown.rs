use crate::{
    controller::{Event, Section, Thread, ToMarkdown},
    server,
    tests::helpers::*,
    DataDB,
};
use serde_json::json;
use std::error::Error;

#[test]
fn event_posted() -> Result<(), Box<dyn Error>> {
    // setup
    let mut client = Client::new();
    let (user_id, user_token) = user::create(&mut client);
    let thread_id = thread::create(&mut client, &user_token);
    let conn = DataDB::get_one(&server()).unwrap();

    // test
    let event_id = client
        .with_base("/v1/event")
        .post(
            Some(&user_token),
            json!({
                "posted": true,
                "cols": [1_546_305_060, "T+0:00", "foo"],
                "in_thread_id": thread_id,
            }),
        )
        .assert_created()
        .get_body_object()["id"]
        .as_i64()
        .unwrap() as i32;

    let md = Event::find_id(&conn, event_id)?.to_markdown(&conn)?;

    assert_eq!("|01:11|T+0:00|foo|\n", md);

    // teardown
    client
        .with_base("/v1/event")
        .delete(Some(&user_token), event_id)
        .assert_no_content();
    thread::delete(&mut client, &user_token, thread_id);
    user::delete(&mut client, user_id);

    Ok(())
}

#[test]
fn event_unposted() -> Result<(), Box<dyn Error>> {
    let database = DataDB::get_one(&server()).unwrap();

    let event = Event {
        id:           0, // irrelevant
        posted:       false,
        cols:         json!([1_546_305_060, "T+0:00", "foo"]),
        in_thread_id: 0, // irrelevant
    };

    let md = event.to_markdown(&database)?;

    assert_eq!("", md);
    Ok(())
}

#[test]
fn section_not_events() -> Result<(), Box<dyn Error>> {
    let database = DataDB::get_one(&server()).unwrap();

    let section = Section {
        id: 0, // irrelevant
        is_events_section: false,
        name: "Introduction".into(),
        content: "foo\n\nbar".into(),
        lock_held_by_user_id: None,
        lock_assigned_at_utc: 0,
        in_thread_id: 0,
    };

    let md = section.to_markdown(&database)?;

    assert_eq!("# Introduction\nfoo\n\nbar", md);
    Ok(())
}

#[test]
fn section_events() -> Result<(), Box<dyn Error>> {
    // setup
    let mut client = Client::new();
    let (user_id, user_token) = user::create(&mut client);
    let thread_id = thread::create(&mut client, &user_token);
    let section_id = client
        .with_base("/v1/section")
        .post(
            Some(&user_token),
            json!({
                "in_thread_id": thread_id,
                "name": "Live Updates",
                "content": "",
                "is_events_section": true,
            }),
        )
        .assert_created()
        .get_body_object()["id"]
        .as_i64()
        .unwrap() as i32;

    let events_id: Vec<_> = vec![
        json!({
            "posted": true,
            "cols": [1_546_305_060, "T+0:00", "foo"],
            "in_thread_id": thread_id,
        }),
        json!({
            "posted": false,
            "cols": [1_546_305_090, "T+0:30", "bar"],
            "in_thread_id": thread_id,
        }),
        json!({
            "posted": true,
            "cols": [1_546_305_120, "T+1:00", "baz"],
            "in_thread_id": thread_id,
        }),
    ]
    .iter()
    .map(|event| {
        client
            .with_base("/v1/event")
            .post(Some(&user_token), event)
            .assert_created()
            .get_body_object()["id"]
            .as_i64()
            .unwrap() as i32
    })
    .collect();

    let conn = crate::DataDB::get_one(&crate::server()).unwrap();

    // test
    let md = Section::find_id(&conn, section_id)?.to_markdown(&conn)?;

    assert_eq!(
        "# Live Updates\n\
         |UTC|Countdown|Update|\n\
         |---|---|---|\n\
         |01:11|T+0:00|foo|\n\
         |01:12|T+1:00|baz|\n\
         ",
        md
    );

    // teardown
    for id in events_id {
        client
            .with_base("/v1/event")
            .delete(Some(&user_token), id)
            .assert_no_content();
    }
    client
        .with_base("/v1/section")
        .delete(Some(&user_token), section_id)
        .assert_no_content();
    thread::delete(&mut client, &user_token, thread_id);
    user::delete(&mut client, user_id);

    Ok(())
}

#[test]
fn thread() -> Result<(), Box<dyn Error>> {
    // setup
    let mut client = Client::new();
    let (user_id, user_token) = user::create(&mut client);
    let thread_id = thread::create(&mut client, &user_token);

    let sections_id: Vec<_> = vec![
        json!({
            "in_thread_id": thread_id,
            "name": "Introduction",
            "content": "Sed consectetur nunc molestie eros.",
            "is_events_section": false,
        }),
        json!({
            "in_thread_id": thread_id,
            "name": "Live Updates",
            "content": "",
            "is_events_section": true,
        }),
        json!({
            "in_thread_id": thread_id,
            "name": "Participate!",
            "content": "Fusce volutpat nisl a metus.",
            "is_events_section": false,
        }),
    ]
    .iter()
    .map(|section| {
        client
            .with_base("/v1/section")
            .post(Some(&user_token), section)
            .assert_created()
            .get_body_object()["id"]
            .as_i64()
            .unwrap() as i32
    })
    .collect();

    let events_id: Vec<_> = vec![
        json!({
            "posted": true,
            "cols": [1_546_305_060, "T+0:00", "foo"],
            "in_thread_id": thread_id,
        }),
        json!({
            "posted": false,
            "cols": [1_546_305_090, "T+0:30", "bar"],
            "in_thread_id": thread_id,
        }),
        json!({
            "posted": true,
            "cols": [1_546_305_120, "T+1:00", "baz"],
            "in_thread_id": thread_id,
        }),
    ]
    .iter()
    .map(|event| {
        client
            .with_base("/v1/event")
            .post(Some(&user_token), event)
            .assert_created()
            .get_body_object()["id"]
            .as_i64()
            .unwrap() as i32
    })
    .collect();

    let conn = crate::DataDB::get_one(&crate::server()).unwrap();

    // test
    let md = Thread::find_id(&conn, thread_id)?.to_markdown(&conn)?;

    assert_eq!(
        "# Introduction\n\
         Sed consectetur nunc molestie eros.\n\
         \n\
         # Live Updates\n\
         |UTC|Countdown|Update|\n\
         |---|---|---|\n\
         |01:11|T+0:00|foo|\n\
         |01:12|T+1:00|baz|\n\
         \n\
         \n\
         # Participate!\n\
         Fusce volutpat nisl a metus.\n\
         \n\
         ",
        md
    );

    // teardown
    for id in events_id {
        client
            .with_base("/v1/event")
            .delete(Some(&user_token), id)
            .assert_no_content();
    }
    for id in sections_id {
        client
            .with_base("/v1/section")
            .delete(Some(&user_token), id)
            .assert_no_content();
    }
    thread::delete(&mut client, &user_token, thread_id);
    user::delete(&mut client, user_id);

    Ok(())
}
