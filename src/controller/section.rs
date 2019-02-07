use super::SECTION_CACHE_SIZE;
use crate::{
    controller::thread::{Thread, UpdateThread},
    schema::section::{self, dsl::*},
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
    static ref CACHE: Mutex<LruCache<i32, Section>> = Mutex::new(LruCache::new(SECTION_CACHE_SIZE));
}

generate_structs! {
    Section("section") {
        auto id: i32,
        readonly is_events_section: bool = false,
        name: String = "",
        content: String = "",
        // not actually auto, but updates are handled by a different struct
        auto lock_held_by_user_id: Option<i32>,
        readonly in_thread_id: i32,
    }
}

#[derive(AsChangeset, Deserialize)]
#[serde(deny_unknown_fields)]
#[table_name = "section"]
pub struct LockSection {
    pub lock_held_by_user_id: Option<i32>,
}

impl Section {
    /// Find all `Section`s in the database.
    ///
    /// Does _not_ use cache (reading or writing),
    /// so as to avoid storing values rarely accessed.
    #[inline]
    pub fn find_all(conn: &Database) -> QueryResult<Vec<Self>> {
        section.load(conn)
    }

    /// Find a given `Section` by its ID.
    ///
    /// Internally uses a cache to limit database accesses.
    #[inline]
    pub fn find_id(conn: &Database, section_id: i32) -> QueryResult<Self> {
        let mut cache = CACHE.lock();
        if cache.contains_key(&section_id) {
            Ok(cache.get_mut(&section_id).unwrap().clone())
        } else {
            let result: Self = section.find(section_id).first(conn)?;
            cache.insert(section_id, result.clone());
            Ok(result)
        }
    }

    /// Create a `Section` given the data.
    ///
    /// The inserted row is added to the global cache and returned.
    #[inline]
    pub fn create(conn: &Database, data: &InsertSection) -> QueryResult<Self> {
        let result: Self = diesel::insert_into(section).values(data).get_result(conn)?;
        CACHE.lock().insert(result.id, result.clone());

        // Add the section ID to the relevant Thread.
        let mut thread = Thread::find_id(conn, data.in_thread_id)?;
        thread.sections_id.push(result.id);
        Thread::update(
            conn,
            data.in_thread_id,
            &UpdateThread {
                sections_id: Some(thread.sections_id),
                ..Default::default()
            },
        )?;

        Ok(result)
    }

    /// Update a `Section` given an ID and the data to update.
    ///
    /// The entry is updated in the database, added to cache, and returned.
    #[inline]
    pub fn update(conn: &Database, section_id: i32, data: &UpdateSection) -> QueryResult<Self> {
        let result: Self = diesel::update(section)
            .filter(id.eq(section_id))
            .set(data)
            .get_result(conn)?;
        CACHE.lock().insert(result.id, result.clone());
        Ok(result)
    }

    /// Set a lock on a `Section`.
    /// Integrity and authority to perform this action is _not_ verified here.
    ///
    /// The entry is updated in the database, added to cache, and returned.
    #[inline]
    pub fn set_lock(conn: &Database, section_id: i32, data: &LockSection) -> QueryResult<Self> {
        let result: Self = diesel::update(section)
            .filter(id.eq(section_id))
            .set(data)
            .get_result(conn)?;
        CACHE.lock().insert(result.id, result.clone());
        Ok(result)
    }

    /// Delete a `Section` given its ID.
    ///
    /// Removes the entry from cache and returns the number of rows deleted (should be `1`).
    #[inline]
    pub fn delete(conn: &Database, section_id: i32) -> QueryResult<usize> {
        let mut thread = Thread::find_id(conn, Section::find_id(conn, section_id)?.in_thread_id)?;
        thread.sections_id.retain(|&cur_id| cur_id != section_id);
        Thread::update(
            conn,
            thread.id,
            &UpdateThread {
                sections_id: Some(thread.sections_id),
                ..Default::default()
            },
        )?;

        CACHE.lock().remove(&section_id);
        diesel::delete(section)
            .filter(id.eq(section_id))
            .execute(conn)
    }
}
