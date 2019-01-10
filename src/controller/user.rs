#![allow(non_snake_case)]

use crate::schema::{user, user::dsl::*};
use diesel::{sql_query, ExpressionMethods};
use rocket_contrib::databases::diesel::{QueryDsl, QueryResult, RunQueryDsl, SqliteConnection};

#[derive(Serialize, Deserialize, Queryable, Clone)]
#[serde(deny_unknown_fields)]
pub struct User {
    pub id: i32,
    pub reddit_username: String,
    pub lang: String,
    #[serde(skip_serializing)]
    pub refresh_token: String,
    pub is_global_admin: bool,
    pub spacex__is_admin: bool,
    pub spacex__is_mod: bool,
    pub spacex__is_slack_member: bool,
}

#[inline(always)]
fn default_lang() -> String {
    String::from("en")
}

#[derive(Deserialize, Insertable)]
#[serde(deny_unknown_fields)]
#[table_name = "user"]
pub struct InsertUser {
    pub reddit_username: String,
    #[serde(default = "default_lang")]
    pub lang: String,
    pub refresh_token: String,
    #[serde(default)]
    pub is_global_admin: bool,
    #[serde(default)]
    pub spacex__is_admin: bool,
    #[serde(default)]
    pub spacex__is_mod: bool,
    #[serde(default)]
    pub spacex__is_slack_member: bool,
}

#[derive(Deserialize, AsChangeset)]
#[serde(deny_unknown_fields)]
#[table_name = "user"]
pub struct UpdateUser {
    pub lang: Option<String>,
    pub refresh_token: Option<String>,
    pub is_global_admin: Option<bool>,
    pub spacex__is_admin: Option<bool>,
    pub spacex__is_mod: Option<bool>,
    pub spacex__is_slack_member: Option<bool>,
}

impl User {
    #[inline]
    pub fn find_all(conn: &SqliteConnection) -> QueryResult<Vec<User>> {
        user.load(conn)
    }

    #[inline]
    pub fn find_id(conn: &SqliteConnection, user_id: i32) -> QueryResult<User> {
        user.find(user_id).first(conn)
    }

    #[inline]
    pub fn create(conn: &SqliteConnection, data: &InsertUser) -> QueryResult<User> {
        diesel::insert_into(user)
            .values(data)
            .execute(conn)
            .map(|_| {
                let user_id = sql_query("SELECT LAST_INSERT_ROWID()")
                    .execute(conn)
                    .expect("Could not get ID of inserted row")
                    as i32;
                User::find_id(conn, user_id).expect("Could not find inserted row")
            })
    }

    #[inline]
    pub fn update(conn: &SqliteConnection, user_id: i32, data: &UpdateUser) -> QueryResult<User> {
        diesel::update(user)
            .filter(id.eq(user_id))
            .set(data)
            .execute(conn)
            .map(|_| User::find_id(conn, user_id).unwrap())
    }

    #[inline]
    pub fn delete(conn: &SqliteConnection, user_id: i32) -> QueryResult<usize> {
        diesel::delete(user).filter(id.eq(user_id)).execute(conn)
    }
}
