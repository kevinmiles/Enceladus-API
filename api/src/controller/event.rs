use super::{Thread, ToMarkdown, UpdateThread, EVENT_CACHE_SIZE};
use crate::{schema::event, Database};
use enceladus_macros::generate_structs;
use lazy_static::lazy_static;
use lru_cache::LruCache;
use parking_lot::Mutex;
use rocket_contrib::databases::diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
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
    /// ```
    static ref CACHE: Mutex<LruCache<i32, Event>> = Mutex::new(LruCache::new(EVENT_CACHE_SIZE));
}

generate_structs! {
    Event("event") {
        auto id: i32,
        posted: bool = false,
        readonly in_thread_id: i32,
        cols: serde_json::Value,
    }
}

impl Event {
    /// Find all `Event`s in the database.
    ///
    /// Does _not_ use cache (reading or writing),
    /// so as to avoid storing values rarely accessed.
    #[inline]
    pub fn find_all(conn: &Database) -> QueryResult<Vec<Self>> {
        use crate::schema::event::dsl::*;

        event.load(conn)
    }

    /// Find a given `Event` by its ID.
    ///
    /// Internally uses a cache to limit database accesses.
    #[inline]
    pub fn find_id(conn: &Database, event_id: i32) -> QueryResult<Self> {
        use crate::schema::event::dsl::*;

        let mut cache = CACHE.lock();
        if cache.contains_key(&event_id) {
            Ok(cache.get_mut(&event_id).unwrap().clone())
        } else {
            let result: Self = event.find(event_id).first(conn)?;
            cache.insert(event_id, result.clone());
            Ok(result)
        }
    }

    /// Create an `Event` given the data.
    ///
    /// The inserted row is added to the global cache and returned.
    #[inline]
    pub fn create(conn: &Database, data: &InsertEvent) -> QueryResult<Self> {
        use crate::schema::event::dsl::*;

        let result: Self = diesel::insert_into(event).values(data).get_result(conn)?;
        CACHE.lock().insert(result.id, result.clone());

        // Add the event ID to the relevant Thread.
        let mut thread = Thread::find_id(conn, data.in_thread_id)?;
        thread.events_id.push(result.id);
        Thread::update(
            conn,
            data.in_thread_id,
            &UpdateThread {
                events_id: Some(thread.events_id),
                ..Default::default()
            },
        )?;

        Ok(result)
    }

    /// Update an `Event` given an ID and the data to update.
    ///
    /// The entry is updated in the database, added to cache, and returned.
    #[inline]
    pub fn update(conn: &Database, event_id: i32, data: &UpdateEvent) -> QueryResult<Self> {
        use crate::schema::event::dsl::*;

        let result: Self = diesel::update(event)
            .filter(id.eq(event_id))
            .set(data)
            .get_result(conn)?;
        CACHE.lock().insert(result.id, result.clone());
        Ok(result)
    }

    /// Delete an `Event` given its ID.
    ///
    /// Removes the entry from cache and returns the number of rows deleted (should be `1`).
    #[inline]
    pub fn delete(conn: &Database, event_id: i32) -> QueryResult<usize> {
        use crate::schema::event::dsl::*;

        let mut thread = Thread::find_id(conn, Event::find_id(conn, event_id)?.in_thread_id)?;
        thread.events_id.retain(|&cur_id| cur_id != event_id);
        Thread::update(
            conn,
            thread.id,
            &UpdateThread {
                events_id: Some(thread.events_id),
                ..Default::default()
            },
        )?;

        CACHE.lock().remove(&event_id);
        diesel::delete(event).filter(id.eq(event_id)).execute(conn)
    }
}

impl ToMarkdown for Event {
    #[inline]
    fn to_markdown(&self, conn: &Database) -> Result<String, Box<dyn Error>> {
        if !self.posted {
            return Ok("".into());
        }

        let mut md = String::new();

        if !self.cols.is_array() {
            panic!("Expected columns to be array");
        }

        let utc_col_index = Thread::find_id(conn, self.in_thread_id)?.space__utc_col_index;

        for (val, i) in self.cols.as_array().unwrap().iter().zip(0..) {
            write!(
                &mut md,
                "|{}",
                // If the column in question is the designated UTC timestamp,
                // format it as such.
                if Some(i) == utc_col_index {
                    let timestamp = val.as_i64().expect("expected i64 in UTC column");
                    let hours = timestamp % 86_400 / 3_600;
                    let minutes = timestamp % 3_600 / 60;

                    format!("{:02}:{:02}", hours, minutes)
                } else {
                    use serde_json::Value::*;
                    match val {
                        Number(ref n) => n.clone().as_i64().unwrap().to_string(),
                        String(ref s) => s.clone().to_owned(),
                        _ => panic!("Expected number or string"),
                    }
                }
                .replace('\n', " ")
                .replace('|', "\\|")
            )?;
        }

        writeln!(&mut md, "|")?;

        Ok(md)
    }
}
