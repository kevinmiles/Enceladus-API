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
    /// Find all `PresetEvent`s in the database and return the result.
    #[inline]
    pub fn find_all(conn: &Database) -> QueryResult<Vec<PresetEvent>> {
        preset_event.load(conn)
    }

    /// Find a specific `PresetEvent` given its ID.
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
    /// Returns the inserted row.
    #[inline]
    pub fn create(conn: &Database, data: &InsertPresetEvent) -> QueryResult<PresetEvent> {
        let result: PresetEvent = diesel::insert_into(preset_event)
            .values(data)
            .get_result(conn)?;
        CACHE.write().insert(result.id, result.clone());
        Ok(result)
    }

    /// Update a `PresetEvent` given an ID and the data to update.
    /// Returns the full row.
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
    /// Returns the number of rows deleted (should be `1`).
    #[inline]
    pub fn delete(conn: &Database, preset_event_id: i32) -> QueryResult<usize> {
        CACHE.write().remove(&preset_event_id);
        diesel::delete(preset_event)
            .filter(id.eq(preset_event_id))
            .execute(conn)
    }
}
