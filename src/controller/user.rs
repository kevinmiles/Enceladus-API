#![allow(non_snake_case)]

use crate::{
    schema::user::{self, dsl::*},
    Database,
};
use enceladus_macros::generate_structs;
use hashbrown::HashMap;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use rocket_contrib::databases::diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

lazy_static! {
    static ref CACHE: RwLock<HashMap<i32, User>> = RwLock::new(HashMap::new());
}

generate_structs! {
    User("user") {
        auto id: i32,
        readonly reddit_username: String,
        lang: String = "en",
        private refresh_token: String,
        is_global_admin: bool = false,
        spacex__is_admin: bool = false,
        spacex__is_mod: bool = false,
        spacex__is_slack_member: bool = false,
    }
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
        let cache = CACHE.read();
        if cache.contains_key(&user_id) {
            Ok(cache[&user_id].clone())
        } else {
            let result: User = user.find(user_id).first(conn)?;
            CACHE.write().insert(user_id, result.clone());
            Ok(result)
        }
    }

    /// Create a `User` given the data.
    /// Returns the inserted row.
    #[inline]
    pub fn create(conn: &Database, data: &InsertUser) -> QueryResult<User> {
        let result: User = diesel::insert_into(user).values(data).get_result(conn)?;
        CACHE.write().insert(result.id, result.clone());
        Ok(result)
    }

    /// Update a `User` given an ID and the data to update.
    /// Returns the full row.
    #[inline]
    pub fn update(conn: &Database, user_id: i32, data: &UpdateUser) -> QueryResult<User> {
        let result: User = diesel::update(user)
            .filter(id.eq(user_id))
            .set(data)
            .get_result(conn)?;
        CACHE.write().insert(result.id, result.clone());
        Ok(result)
    }

    /// Delete a `PresetEvent` given its ID.
    /// Returns the number of rows deleted (should be `1`).
    #[inline]
    pub fn delete(conn: &Database, user_id: i32) -> QueryResult<usize> {
        CACHE.write().remove(&user_id);
        diesel::delete(user).filter(id.eq(user_id)).execute(conn)
    }
}
