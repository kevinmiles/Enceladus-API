#![allow(non_snake_case)]

use crate::{
    schema::{user, user::dsl::*},
    Database,
};
use enceladus_macros::{InsertStruct, UpdateStruct};
use rocket_contrib::databases::diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

#[derive(Serialize, Deserialize, Queryable, Clone, InsertStruct, UpdateStruct)]
#[table_name = "user"]
#[serde(deny_unknown_fields)]
pub struct User {
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
    #[inline]
    pub fn find_all(conn: &Database) -> QueryResult<Vec<User>> {
        user.load(conn)
    }

    #[inline]
    pub fn find_id(conn: &Database, user_id: i32) -> QueryResult<User> {
        user.find(user_id).first(conn)
    }

    #[inline]
    pub fn create(conn: &Database, data: &InsertUser) -> QueryResult<User> {
        diesel::insert_into(user)
            .values(data)
            .execute(conn)
            .map(|_| find_inserted!(user, conn))
    }

    #[inline]
    pub fn update(conn: &Database, user_id: i32, data: &UpdateUser) -> QueryResult<User> {
        diesel::update(user)
            .filter(id.eq(user_id))
            .set(data)
            .execute(conn)
            .map(|_| User::find_id(conn, user_id).unwrap())
    }

    #[inline]
    pub fn delete(conn: &Database, user_id: i32) -> QueryResult<usize> {
        diesel::delete(user).filter(id.eq(user_id)).execute(conn)
    }
}
