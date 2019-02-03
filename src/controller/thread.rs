#![allow(non_snake_case)]

use crate::{
    schema::thread::{self, dsl::*},
    Database,
};
use enceladus_macros::generate_structs;
use lazy_static::lazy_static;
use lru_cache::LruCache;
use parking_lot::Mutex;
use rocket_contrib::databases::diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use serde::Deserialize;

lazy_static! {
    /// A global cache, containing a mapping of IDs to their respective `Event`.
    ///
    /// The cache is protected by a `Mutex`,
    /// ensuring there is only ever at most one writer at a time.
    /// Note that even when reading,
    /// there must be a lock on mutability,
    /// as the `LruCache` must be able to update itself.
    ///
    /// To read from the cache,
    /// you'll want to call `CACHE.lock()` before performing normal operations.
    /// ```
    static ref CACHE: Mutex<LruCache<i32, Thread>> = Mutex::new(LruCache::new(5));
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

// Not all fields that are insertable should be provided by the user.
// Use an `ExternalInsertThread` wherever user input is expected.
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
    /// Find all `Thread`s in the database.
    ///
    /// Does _not_ use cache (reading or writing),
    /// so as to avoid storing values rarely accessed.
    #[inline]
    pub fn find_all(conn: &Database) -> QueryResult<Vec<Self>> {
        thread.load(conn)
    }

    /// Find a given `Thread` by its ID.
    ///
    /// Internally uses a cache to limit database accesses.
    #[inline]
    pub fn find_id(conn: &Database, thread_id: i32) -> QueryResult<Self> {
        let mut cache = CACHE.lock();
        if cache.contains_key(&thread_id) {
            Ok(cache.get_mut(&thread_id).unwrap().clone())
        } else {
            let result: Self = thread.find(thread_id).first(conn)?;
            cache.insert(thread_id, result.clone());
            Ok(result)
        }
    }

    /// Create a `Thread` given the data.
    ///
    /// The inserted row is added to the global cache and returned.
    #[inline]
    pub fn create(conn: &Database, data: &ExternalInsertThread, user_id: i32) -> QueryResult<Self> {
        let insertable_thread = InsertThread {
            thread_name: data.thread_name.clone(),
            launch_name: data.launch_name.clone(),
            post_id: None, // temporary
            subreddit: data.subreddit.clone(),
            t0: data.t0,
            youtube_id: data.youtube_id.clone(),
            spacex__api_id: data.spacex__api_id.clone(),
            created_by_user_id: user_id,
            events_id: vec![],
            sections_id: vec![],
        };

        let result: Self = diesel::insert_into(thread)
            .values(insertable_thread)
            .get_result(conn)?;
        CACHE.lock().insert(result.id, result.clone());
        Ok(result)
    }

    /// Update a `Thread` given an ID and the data to update.
    ///
    /// The entry is updated in the database, added to cache, and returned.
    #[inline]
    pub fn update(conn: &Database, thread_id: i32, data: &UpdateThread) -> QueryResult<Self> {
        let result: Self = diesel::update(thread)
            .filter(id.eq(thread_id))
            .set(data)
            .get_result(conn)?;
        CACHE.lock().insert(result.id, result.clone());
        Ok(result)
    }

    /// Delete a `Thread` given its ID.
    ///
    /// Removes the entry from cache and returns the number of rows deleted (should be `1`).
    #[inline]
    pub fn delete(conn: &Database, thread_id: i32) -> QueryResult<usize> {
        CACHE.lock().remove(&thread_id);
        diesel::delete(thread)
            .filter(id.eq(thread_id))
            .execute(conn)
    }
}
