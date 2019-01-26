#![allow(non_snake_case)]

use crate::{
    schema::thread::{self, dsl::*},
    Database,
};
use enceladus_macros::generate_structs;
use rocket_contrib::databases::diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use serde::Deserialize;

generate_structs! {
    Thread("thread") {
        auto id: i32,
        readonly thread_name: String,
        launch_name: String,
        readonly post_id: Option<String>,
        readonly subreddit: String,
        t0: Option<i64>,
        youtube_id: Option<String>,
        spacex__api_id: Option<String>,
        readonly created_by_user_id: i32,
        sections_id: Vec<i32> = vec![],
        events_id: Vec<i32> = vec![],
    }
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
            events_id: vec![],
            sections_id: vec![],
        };

        diesel::insert_into(thread)
            .values(insertable_thread)
            .get_result(conn)
    }

    #[inline]
    pub fn update(conn: &Database, thread_id: i32, data: &UpdateThread) -> QueryResult<Thread> {
        diesel::update(thread)
            .filter(id.eq(thread_id))
            .set(data)
            .get_result(conn)
    }

    #[inline]
    pub fn delete(conn: &Database, thread_id: i32) -> QueryResult<usize> {
        diesel::delete(thread)
            .filter(id.eq(thread_id))
            .execute(conn)
    }
}
