use super::PRESET_EVENT_CACHE_SIZE;
use crate::{
    schema::preset_event::{self, dsl::*},
    Database,
};
use enceladus_macros::generate_structs;
use lazy_static::lazy_static;
use lru_cache::LruCache;
use parking_lot::Mutex;
use rocket_contrib::databases::diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

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
    static ref CACHE: Mutex<LruCache<i32, PresetEvent>> = Mutex::new(LruCache::new(PRESET_EVENT_CACHE_SIZE));
}

generate_structs! {
    PresetEvent("preset_event") {
        auto id: i32,
        holds_clock: bool = false,
        message: String,
        name: String,
    }
}

impl PresetEvent {
    /// Find all `PresetEvent`s in the database.
    ///
    /// Does _not_ use cache (reading or writing),
    /// so as to avoid storing values rarely accessed.
    #[inline]
    pub fn find_all(conn: &Database) -> QueryResult<Vec<Self>> {
        preset_event.load(conn)
    }

    /// Find a given `PresetEvent` by its ID.
    ///
    /// Internally uses a cache to limit database accesses.
    #[inline]
    pub fn find_id(conn: &Database, preset_event_id: i32) -> QueryResult<Self> {
        let mut cache = CACHE.lock();
        if cache.contains_key(&preset_event_id) {
            Ok(cache.get_mut(&preset_event_id).unwrap().clone())
        } else {
            let result: Self = preset_event.find(preset_event_id).first(conn)?;
            cache.insert(preset_event_id, result.clone());
            Ok(result)
        }
    }

    /// Create a `PresetEvent` given the data.
    ///
    /// The inserted row is added to the global cache and returned.
    #[inline]
    pub fn create(conn: &Database, data: &InsertPresetEvent) -> QueryResult<Self> {
        let result: Self = diesel::insert_into(preset_event)
            .values(data)
            .get_result(conn)?;
        CACHE.lock().insert(result.id, result.clone());
        Ok(result)
    }

    /// Update a `PresetEvent` given an ID and the data to update.
    ///
    /// The entry is updated in the database, added to cache, and returned.
    #[inline]
    pub fn update(
        conn: &Database,
        preset_event_id: i32,
        data: &UpdatePresetEvent,
    ) -> QueryResult<Self> {
        let result: Self = diesel::update(preset_event)
            .filter(id.eq(preset_event_id))
            .set(data)
            .get_result(conn)?;
        CACHE.lock().insert(result.id, result.clone());
        Ok(result)
    }

    /// Delete a `PresetEvent` given its ID.
    ///
    /// Removes the entry from cache and returns the number of rows deleted (should be `1`).
    #[inline]
    pub fn delete(conn: &Database, preset_event_id: i32) -> QueryResult<usize> {
        CACHE.lock().remove(&preset_event_id);
        diesel::delete(preset_event)
            .filter(id.eq(preset_event_id))
            .execute(conn)
    }
}
