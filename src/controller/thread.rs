#![allow(non_snake_case)]

use crate::{
    schema::thread::{self, dsl::*},
    Database,
};
use enceladus_macros::generate_structs;
use hashbrown::HashMap;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use rocket_contrib::databases::diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use serde::Deserialize;

lazy_static! {
    static ref CACHE: RwLock<HashMap<i32, Thread>> = RwLock::new(HashMap::new());
}

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
        let cache = CACHE.read();
        if cache.contains_key(&thread_id) {
            Ok(cache[&thread_id].clone())
        } else {
            let result: Thread = thread.find(thread_id).first(conn)?;
            CACHE.write().insert(thread_id, result.clone());
            Ok(result)
        }
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

        let result: Thread = diesel::insert_into(thread)
            .values(insertable_thread)
            .get_result(conn)?;
        CACHE.write().insert(result.id, result.clone());
        Ok(result)
    }

    #[inline]
    pub fn update(conn: &Database, thread_id: i32, data: &UpdateThread) -> QueryResult<Thread> {
        let result: Thread = diesel::update(thread)
            .filter(id.eq(thread_id))
            .set(data)
            .get_result(conn)?;
        CACHE.write().insert(result.id, result.clone());
        Ok(result)
    }

    #[inline]
    pub fn delete(conn: &Database, thread_id: i32) -> QueryResult<usize> {
        CACHE.write().remove(&thread_id);
        diesel::delete(thread)
            .filter(id.eq(thread_id))
            .execute(conn)
    }
}
