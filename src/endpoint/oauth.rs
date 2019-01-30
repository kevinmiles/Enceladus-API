use crate::{
    controller::user::{InsertUser, User},
    guid, DataDB,
};
use jsonwebtoken as jwt;
use rocket::{get, http::RawStr, response::Redirect, uri};
use std::error::Error;
use url::Url;

// TODO store this data in cookies,
// such that we can avoid going to reddit
// if another app asks for authentication

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

    if cfg!(test) {
        println!("point 1");
    }

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
    let token = jwt::encode(
        &jwt::Header::default(),
        &user.id,
        std::env::var("ROCKET_SECRET_KEY").unwrap().as_bytes(),
    )?;

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

    if cfg!(test) {
        println!("point 4");
    }

    Ok(Redirect::to(callback.to_string()))
}
