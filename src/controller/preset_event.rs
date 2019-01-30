use crate::{
    schema::preset_event::{self, dsl::*},
    Database,
};
use enceladus_macros::generate_structs;
use hashbrown::HashMap;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use rocket_contrib::databases::diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

lazy_static! {
    /// A global cache, containing a mapping of IDs to their respective `PresetEvent`.
    ///
    /// The cache is protected by a `RwLock`,
    /// ensuring there is only ever at most one writer (and no readers) at a point in time.
    ///
    /// To read from the cache,
    /// you'll want to call `CACHE.read()` before performing normal operations.
    /// The same is true for `CACHE.write()`.
    ///
    /// It is _highly_ recommended to manually call `drop()` after you're done using the lock.
    /// This ensures that nothing else is blocked from accessing the cache if necessary.
    ///
    /// Here's example of when this is necessary to ensure working code:
    ///
    /// ```rust
    /// // Obtain a read lock on the global cache.
    /// let cache = CACHE.read();
    ///
    /// if cache.contains_key("foo") {
    ///     // Do something with the value.
    ///     cache["foo"]
    /// } else {
    ///     // Manually drop the `cache` variable,
    ///     // letting us obtain a write lock.
    ///     std::mem::drop(cache);
    ///
    ///     // Now we can obtain a write lock without having to wait
    ///     // for the read lock to be dropped automatically.
    ///     // Note that this _would not happen_ until _after_ the request for the write lock,
    ///     // causing a deadlock in the code not caught by the compiler.
    ///     CACHE.write().insert("foo", "bar");
    /// }
    /// ```
    static ref CACHE: RwLock<HashMap<i32, PresetEvent>> = RwLock::new(HashMap::new());
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
    pub fn find_all(conn: &Database) -> QueryResult<Vec<PresetEvent>> {
        preset_event.load(conn)
    }

    /// Find a given `PresetEvent` by its ID.
    ///
    /// Internally uses a cache to limit database accesses.
    #[inline]
    pub fn find_id(conn: &Database, preset_event_id: i32) -> QueryResult<PresetEvent> {
        let cache = CACHE.read();
        if cache.contains_key(&preset_event_id) {
            Ok(cache[&preset_event_id].clone())
        } else {
            // drop the read lock on the cache,
            // ensuring we can call `CACHE.write()` without issue
            std::mem::drop(cache);

            let result: PresetEvent = preset_event.find(preset_event_id).first(conn)?;
            CACHE.write().insert(preset_event_id, result.clone());
            Ok(result)
        }
    }

    /// Create a `PresetEvent` given the data.
    ///
    /// The inserted row is added to the global cache and returned.
    #[inline]
    pub fn create(conn: &Database, data: &InsertPresetEvent) -> QueryResult<PresetEvent> {
        let result: PresetEvent = diesel::insert_into(preset_event)
            .values(data)
            .get_result(conn)?;
        CACHE.write().insert(result.id, result.clone());
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
    ) -> QueryResult<PresetEvent> {
        let result: PresetEvent = diesel::update(preset_event)
            .filter(id.eq(preset_event_id))
            .set(data)
            .get_result(conn)?;
        CACHE.write().insert(result.id, result.clone());
        Ok(result)
    }

    /// Delete a `PresetEvent` given its ID.
    ///
    /// Removes the entry from cache and returns the number of rows deleted (should be `1`).
    #[inline]
    pub fn delete(conn: &Database, preset_event_id: i32) -> QueryResult<usize> {
        CACHE.write().remove(&preset_event_id);
        diesel::delete(preset_event)
            .filter(id.eq(preset_event_id))
            .execute(conn)
    }
}
