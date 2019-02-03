#![allow(non_snake_case)]

use crate::controller::user::User;

#[cfg(test)]
use crate::{
    controller::{
        claim::Claim,
        user::{InsertUser, UpdateUser},
    },
    endpoint::helpers::RocketResult,
    DataDB,
};

#[cfg(test)]
use rocket::{post, response::status::Created};

#[cfg(test)]
use rocket_contrib::json::Json;

#[cfg(test)]
use serde::Serialize;

#[cfg(test)]
use std::convert::From;

generic_all!(User);
generic_get!(User);

#[cfg(test)]
generic_patch!(User);

#[cfg(test)]
generic_delete!(User);

#[cfg(test)]
#[inline]
#[post("/", data = "<data>")]
pub fn post(conn: DataDB, data: Json<InsertUser>) -> RocketResult<Created<Json<TokenUser>>> {
    created!(User::create(&conn, &data).map(TokenUser::from))
}

// There's no need to use this elsewhere,
// as this struct exists solely to make testing easier.
#[cfg(test)]
#[derive(Serialize)]
pub struct TokenUser {
    token: String,
    id: i32,
    reddit_username: String,
    lang: String,
    is_global_admin: bool,
    spacex__is_admin: bool,
    spacex__is_mod: bool,
    spacex__is_slack_member: bool,
}

#[cfg(test)]
impl From<User> for TokenUser {
    fn from(user: User) -> TokenUser {
        TokenUser {
            token: Claim::new(user.id).encode().unwrap(),
            id: user.id,
            reddit_username: user.reddit_username,
            lang: user.lang,
            is_global_admin: user.is_global_admin,
            spacex__is_admin: user.spacex__is_admin,
            spacex__is_mod: user.spacex__is_mod,
            spacex__is_slack_member: user.spacex__is_slack_member,
        }
    }
}
