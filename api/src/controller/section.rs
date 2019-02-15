use super::{Event, Thread, ToMarkdown, UpdateThread, SECTION_CACHE_SIZE};
use crate::{
    schema::section::{self, dsl::*},
    Database,
};
use enceladus_macros::generate_structs;
use lazy_static::lazy_static;
use lru_cache::LruCache;
use parking_lot::Mutex;
use rocket_contrib::databases::diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use serde::Deserialize;
use std::{error::Error, fmt::Write};

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

// Fields relating to the lock are not necessarily `auto`,
// but are declared as such as they are handled by the `LockSection` struct.
generate_structs! {
    Section("section") {
        auto id: i32,
        readonly is_events_section: bool = false,
        name: String = "",
        content: String = "",
        auto lock_held_by_user_id: Option<i32>,
        readonly in_thread_id: i32,
        auto lock_assigned_at_utc: i64,
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalLockSection {
    pub lock_held_by_user_id: Option<i32>,
}

#[derive(AsChangeset)]
#[table_name = "section"]
pub struct LockSection {
    pub lock_held_by_user_id: Option<i32>,
    pub lock_assigned_at_utc: i64,
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

impl ToMarkdown for Section {
    #[inline]
    fn to_markdown(&self, conn: &Database) -> Result<String, Box<dyn Error>> {
        let mut md = String::new();

        writeln!(&mut md, "# {}", self.name)?;

        if self.is_events_section {
            writeln!(&mut md, "|UTC|Countdown|Update|")?;
            writeln!(&mut md, "|---|---|---|")?;

            let events = Thread::find_id(conn, self.in_thread_id)?.events_id;

            for &event_id in events.iter() {
                write!(
                    &mut md,
                    "{}",
                    Event::find_id(conn, event_id)?.to_markdown(conn)?
                )?;
            }
        } else {
            write!(&mut md, "{}", self.content)?;
        }

        Ok(md)
    }
}
