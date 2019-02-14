use crate::{guid, tests::helpers::*};
use serde_json::json;

#[allow(dead_code)]
const BASE: &str = "/v1/thread";

#[allow(dead_code)]
pub fn create(client: &mut Client, token: impl ToString) -> i32 {
    let response = client
        .with_base(BASE)
        .post(
            Some(&token.to_string()),
            json!({
                "thread_name": guid(),
                "display_name": guid(),
                "subreddit": guid(),
            }),
        )
        .assert_created()
        .get_body_object();

    response["id"].as_i64().unwrap() as i32
}

#[allow(dead_code)]
pub fn delete(client: &mut Client, token: impl ToString, id: i32) {
    client.with_base(BASE).delete(Some(&token.to_string()), id);
}
