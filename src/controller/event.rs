use crate::{
    schema::event::{self, dsl::*},
    Database,
};
use enceladus_macros::generate_structs;
use hashbrown::HashMap;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use rocket_contrib::databases::diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

lazy_static! {
    /// A global cache, containing a mapping of IDs to their respective `Event`.
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
    static ref CACHE: RwLock<HashMap<i32, Event>> = RwLock::new(HashMap::new());
}

generate_structs! {
    Event("event") {
        auto id: i32,
        posted: bool = false,
        message: String = "",
        terminal_count: String = "",
        utc: i64,
        readonly in_thread_id: i32,
    }
}

impl Event {
    /// Find all `Event`s in the database.
    ///
    /// Does _not_ use cache (reading or writing),
    /// so as to avoid storing values rarely accessed.
    #[inline]
    pub fn find_all(conn: &Database) -> QueryResult<Vec<Self>> {
        event.load(conn)
    }

    /// Find a given `Event` by its ID.
    ///
    /// Internally uses a cache to limit database accesses.
    #[inline]
    pub fn find_id(conn: &Database, event_id: i32) -> QueryResult<Self> {
        let cache = CACHE.read();
        if cache.contains_key(&event_id) {
            Ok(cache[&event_id].clone())
        } else {
            // drop the read lock on the cache,
            // ensuring we can call `CACHE.write()` without issue
            std::mem::drop(cache);

            let result: Self = event.find(event_id).first(conn)?;
            CACHE.write().insert(event_id, result.clone());
            Ok(result)
        }
    }

    /// Create an `Event` given the data.
    ///
    /// The inserted row is added to the global cache and returned.
    #[inline]
    pub fn create(conn: &Database, data: &InsertEvent) -> QueryResult<Self> {
        let result: Self = diesel::insert_into(event).values(data).get_result(conn)?;
        CACHE.write().insert(result.id, result.clone());
        Ok(result)
    }

    /// Update an `Event` given an ID and the data to update.
    ///
    /// The entry is updated in the database, added to cache, and returned.
    #[inline]
    pub fn update(conn: &Database, event_id: i32, data: &UpdateEvent) -> QueryResult<Self> {
        let result: Self = diesel::update(event)
            .filter(id.eq(event_id))
            .set(data)
            .get_result(conn)?;
        CACHE.write().insert(result.id, result.clone());
        Ok(result)
    }

    /// Delete an `Event` given its ID.
    ///
    /// Removes the entry from cache and returns the number of rows deleted (should be `1`).
    #[inline]
    pub fn delete(conn: &Database, event_id: i32) -> QueryResult<usize> {
        CACHE.write().remove(&event_id);
        diesel::delete(event).filter(id.eq(event_id)).execute(conn)
    }
}
