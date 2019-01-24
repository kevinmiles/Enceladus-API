#![allow(non_snake_case)]

use crate::{
    schema::thread::{self, dsl::*},
    Database,
};
use enceladus_macros::{InsertStruct, UpdateStruct};
use rocket_contrib::databases::diesel::{
    ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, InsertStruct, UpdateStruct)]
#[table_name = "thread"]
#[serde(deny_unknown_fields)]
pub struct Thread {
    #[no_insert]
    #[no_update]
    pub id: i32,

    #[no_update]
    pub thread_name: String,

    pub launch_name: String,

    #[no_update]
    pub post_id: Option<String>,

    #[no_update]
    pub subreddit: String,

    pub t0: Option<i64>,
    pub youtube_id: Option<String>,
    pub spacex__api_id: Option<String>,

    #[no_update]
    pub created_by_user_id: i32,

    #[no_insert]
    pub sections_id: Vec<i32>,

    #[no_insert]
    pub events_id: Vec<i32>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalInsertThread {
    pub thread_name: String,
    pub launch_name: String,
    pub subreddit: String,
    pub t0: Option<i64>,
    pub youtube_id: Option<String>,
    pub spacex__api_id: Option<String>,
}

impl Thread {
    #[inline]
    pub fn find_all(conn: &Database) -> QueryResult<Vec<Thread>> {
        thread.load(conn)
    }

    #[inline]
    pub fn find_id(conn: &Database, thread_id: i32) -> QueryResult<Thread> {
        thread.find(thread_id).first(conn)
    }

    #[inline]
    pub fn create(conn: &Database, data: &ExternalInsertThread) -> QueryResult<Thread> {
        let insertable_thread = InsertThread {
            thread_name: data.thread_name.clone(),
            launch_name: data.launch_name.clone(),
            post_id: None, // temporary
            subreddit: data.subreddit.clone(),
            t0: data.t0,
            youtube_id: data.youtube_id.clone(),
            spacex__api_id: data.spacex__api_id.clone(),
            created_by_user_id: 0, // temporary
        };

        diesel::insert_into(thread)
            .values(insertable_thread)
            .execute(conn)
            .map(|_| find_inserted!(thread, conn))
    }

    #[inline]
    pub fn update(conn: &Database, thread_id: i32, data: &UpdateThread) -> QueryResult<Thread> {
        diesel::update(thread)
            .filter(id.eq(thread_id))
            .set(data)
            .execute(conn)
            .map(|_| Thread::find_id(conn, thread_id).unwrap())
    }

    #[inline]
    pub fn delete(conn: &Database, thread_id: i32) -> QueryResult<usize> {
        diesel::delete(thread)
            .filter(id.eq(thread_id))
            .execute(conn)
    }
}
