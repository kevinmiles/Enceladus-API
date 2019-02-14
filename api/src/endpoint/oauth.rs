use crate::{
    controller::{
        claim::Claim,
        user::{InsertUser, User},
    },
    guid,
    reddit::RedditUser,
    DataDB,
};
use request::Url;
use reqwest as request;
use rocket::{get, http::RawStr, response::Redirect, uri};
use std::error::Error;

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
pub fn oauth(callback: &RawStr) -> Result<Redirect, Box<Error>> {
    let callback = callback.to_string();

    if cfg!(test) {
        // We're testing; let's not bother with actual authentication.
        // Instead, pretend it succeeded and immediately redirect to the callback.
        Ok(Redirect::to(uri!(
            "/oauth",
            callback: code = guid(),
            state = &callback
        )))
    } else {
        // Send the user off to Reddit for authentication
        Ok(Redirect::to(RedditUser::get_auth_url(&callback)?))
    }
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
    let reddit_user = RedditUser::obtain_refresh_token(&code)?;
    let username = reddit_user.username()?;
    let lang = reddit_user.lang()?;

    // Insert the user into our database.
    let user = User::create(
        &conn,
        &InsertUser {
            reddit_username: username.to_owned(),
            lang: lang.to_owned(),
            refresh_token: reddit_user.refresh_token.clone(),
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
        &state, // The state doubles as our callback.
        &[
            ("user_id", &user.id.to_string()),
            ("username", &username),
            ("lang", &lang),
            ("token", &token),
        ],
    )?;

    Ok(Redirect::to(callback.to_string()))
}
