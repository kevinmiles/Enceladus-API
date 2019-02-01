use crate::{
    controller::{
        claim::Claim,
        user::{InsertUser, User},
    },
    guid, DataDB,
};
use rocket::{get, http::RawStr, response::Redirect, uri};
use std::error::Error;
use url::Url;

// TODO store this data in cookies,
// such that we can avoid going to reddit
// if another app asks for authentication

/// Endpoint that redirects the user to Reddit,
/// requesting to provided permissions.
///
/// In testing, this endpoint immediately forwards to `callback()`
/// in order to avoid any user input or external requests.
#[inline]
#[get("/?<callback>")]
pub fn oauth(callback: &RawStr) -> Redirect {
    let callback = callback.to_string();

    // We're testing; let's not bother with actual authentication.
    // Instead, pretend it succeeded and immediately redirect to the callback.
    if cfg!(test) {
        return Redirect::to(uri!("/oauth", callback: code = guid(), state = &callback));
    }

    // TODO
    // Send the user off to reddit for authentication
    Redirect::to(uri!("/oauth", callback: code = guid(), state = &callback))
}

/// Handle the OAuth response from Reddit.
///
/// Here, we are provided with a refresh token in response to the external OAuth request,
/// and use that token to obtain the user's username and preferred language.
/// All of these values are then used to construct a User
/// which is inserted into the database.
/// We then use the ID returned from the database insertion to generate a
/// [JSON Web Token](https://jwt.io/), which is the user's bearer token
/// that should be provided in the header of each request.
/// Finally, we call the callback URL originally provided,
/// with the additional queryparams of `user_id`, `username`, `lang`, and `token`.
///
/// Please note that `is_global_admin`, along with any specific subreddit values,
/// are **not** initialized, but rather use default values.
/// In the future, there may be an endpoint in the `/v1/user` namespace (or similar)
/// that will automatically perform verification of moderator status
/// (or anything else deemed appropriate).
/// Until that time,
/// these fields must be managed manually,
/// typically by contacting the database operator.
#[inline]
#[get("/callback?<code>&<state>")]
pub fn callback(conn: DataDB, code: String, state: String) -> Result<Redirect, Box<Error>> {
    let original_callback = state;

    // Find the user's username.
    // Fake it if we're only testing.
    let username = match cfg!(test) {
        true => guid(),
        false => guid(),
    };

    // Find the user's language preference.
    // Fake it if we're only testing.
    let lang = match cfg!(test) {
        true => guid()[0..2].to_string(),
        false => guid()[0..2].to_string(),
    };

    // Insert the user into our database.
    let user = User::create(
        &conn,
        &InsertUser {
            reddit_username: username.to_string(),
            lang: lang.to_string(),
            refresh_token: code,
            is_global_admin: false,
            spacex__is_admin: false,
            spacex__is_mod: false,
            spacex__is_slack_member: false,
        },
    )?;

    // Give the user a token that should be used in the future.
    let token = Claim::new(user.id).encode()?;

    // Attach additional querystring parameters to the provided callback.
    let callback = Url::parse_with_params(
        &original_callback,
        &[
            ("user_id", &user.id.to_string()),
            ("username", &username),
            ("lang", &lang),
            ("token", &token),
        ],
    )?;

    Ok(Redirect::to(callback.to_string()))
}
