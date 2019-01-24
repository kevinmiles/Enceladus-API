#![allow(non_snake_case)]

use crate::{
    schema::user::{self, dsl::*},
    Database,
};
use enceladus_macros::{InsertStruct, UpdateStruct};
use rocket_contrib::databases::diesel::{
    ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};

/// Type containing all fields for users.
/// `InsertUser` and `UpdateUser` are automatically derived.
#[derive(Serialize, Deserialize, Queryable, InsertStruct, UpdateStruct)]
#[table_name = "user"]
#[serde(deny_unknown_fields)]
pub struct User {
    #[no_insert]
    #[no_update]
    pub id: i32,
    #[no_update]
    pub reddit_username: String,
    #[insert_default = "en"]
    pub lang: String,
    #[serde(skip_serializing)]
    pub refresh_token: String,
    #[insert_default]
    pub is_global_admin: bool,
    #[insert_default]
    pub spacex__is_admin: bool,
    #[insert_default]
    pub spacex__is_mod: bool,
    #[insert_default]
    pub spacex__is_slack_member: bool,
}

impl User {
    /// Find all `User`s present in the database and return the result.
    #[inline]
    pub fn find_all(conn: &Database) -> QueryResult<Vec<User>> {
        user.load(conn)
    }

    /// Find a specific `User` given its ID.
    #[inline]
    pub fn find_id(conn: &Database, user_id: i32) -> QueryResult<User> {
        user.find(user_id).first(conn)
    }

    /// Create a `User` given the data.
    /// Returns the inserted row.
    #[inline]
    pub fn create(conn: &Database, data: &InsertUser) -> QueryResult<User> {
        diesel::insert_into(user)
            .values(data)
            .execute(conn)
            .map(|_| find_inserted!(user, conn))
    }

    /// Update a `User` given an ID and the data to update.
    /// Returns the full row.
    #[inline]
    pub fn update(conn: &Database, user_id: i32, data: &UpdateUser) -> QueryResult<User> {
        diesel::update(user)
            .filter(id.eq(user_id))
            .set(data)
            .execute(conn)
            .map(|_| User::find_id(conn, user_id).unwrap())
    }

    /// Delete a `PresetEvent` given its ID.
    /// Returns the number of rows deleted (should be `1`).
    #[inline]
    pub fn delete(conn: &Database, user_id: i32) -> QueryResult<usize> {
        diesel::delete(user).filter(id.eq(user_id)).execute(conn)
    }
}
