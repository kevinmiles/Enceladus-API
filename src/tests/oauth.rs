use crate::tests::common::*;
use std::collections::HashMap;
use url::Url;

#[test]
fn returns_auth_data() {
    let client = Client::new("");

    // Simulate what the server performs.
    // This functionality has been confirmed manually.
    //
    // The exact callback URL is irrelevant,
    // it just has to be valid (enforced by the URL crate).
    let callback_redirect = client
        .get("oauth?callback=https://example.com")
        .assert_see_other()
        .get_redirect_uri();
    let client_redirect = client
        .get(callback_redirect)
        .assert_see_other()
        .get_redirect_uri();

    // Confirm valid data on the client's perspective.
    let response_url = Url::parse(&client_redirect).unwrap();
    let auth_data: HashMap<_, _> = response_url.query_pairs().collect();

    // Ensure the appropriate keys are present.
    assert_eq!(auth_data.len(), 4);
    assert!(auth_data.contains_key("user_id"));
    assert!(auth_data.contains_key("username"));
    assert!(auth_data.contains_key("lang"));
    assert!(auth_data.contains_key("token"));
}
