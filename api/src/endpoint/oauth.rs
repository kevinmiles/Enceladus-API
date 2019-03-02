use crate::{
    controller::{Claim, InsertUser, User},
    encryption::encrypt,
    guid,
    DataDB,
};
use lazy_static::lazy_static;
use reddit::Reddit;
use request::Url;
use reqwest as request;
use rocket::{get, http::RawStr, response::Redirect, uri};
use std::{
    error::Error,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

lazy_static! {
    // FIXME make this a regular `const` or `static` once `Option::unwrap` becomes a `const fn`.
    pub static ref REDDIT: Reddit<'static> = Reddit::builder()
        .with_redirect_uri(dotenv!("REDDIT_REDIRECT_URI"))
        .with_user_agent(dotenv!("REDDIT_USER_AGENT"))
        .with_client_id(dotenv!("REDDIT_CLIENT_ID"))
        .with_secret(dotenv!("REDDIT_SECRET"))
        .with_permanent(true)
        .with_scopes({
            use reddit::Scope::*;
            &[
                Account,  // Find language
                Identity, // Find username
                Submit,   // Submit threads
                Edit,     // Update threads
                ModPosts, // (Moderators) Approve a post so it's visible
                ModFlair, // (Moderators) Add/remove/edit a flair on the submission
            ]
        })
        .build()
        .unwrap();
}

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
pub fn oauth(callback: &RawStr) -> Result<Redirect, Box<dyn Error>> {
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
        Ok(Redirect::to(REDDIT.get_auth_url(&callback)?))
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
pub fn callback(conn: DataDB, code: String, state: String) -> Result<Redirect, Box<dyn Error>> {
    let mut reddit_user;
    let lang;
    let username;

    if cfg!(test) {
        reddit_user = reddit::User::builder()
            .with_reddit_instance(&REDDIT)
            .with_refresh_token(guid())
            .with_access_token(guid())
            .with_expires_at(SystemTime::now() + Duration::from_secs(3600))
            .build()
            .unwrap();
        lang = guid()[0..2].to_owned();
        username = guid();
    } else {
        reddit_user = REDDIT.obtain_refresh_token(&code)?;
        username = reddit_user.username()?;
        lang = reddit_user.lang()?;
    }

    // Insert the user into our database.
    let user = User::create(
        &conn,
        &InsertUser {
            reddit_username: username.to_owned(),
            lang: lang.to_owned(),
            refresh_token: encrypt(reddit_user.refresh_token().as_ref()),
            is_global_admin: false,
            spacex__is_admin: false,
            spacex__is_mod: false,
            spacex__is_slack_member: false,
            access_token: encrypt(reddit_user.access_token().as_ref()),
            access_token_expires_at_utc: reddit_user
                .expires_at()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
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
