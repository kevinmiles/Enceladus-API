use crate::{
    schema::preset_event::{self, dsl::*},
    Database,
};
use enceladus_macros::generate_structs;
use rocket_contrib::databases::diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

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
        preset_event.find(preset_event_id).first(conn)
    }

    /// Create a `PresetEvent` given the data.
    /// Returns the inserted row.
    #[inline]
    pub fn create(conn: &Database, data: &InsertPresetEvent) -> QueryResult<PresetEvent> {
        diesel::insert_into(preset_event)
            .values(data)
            .get_result(conn)
    }

    /// Update a `PresetEvent` given an ID and the data to update.
    /// Returns the full row.
    #[inline]
    pub fn update(
        conn: &Database,
        preset_event_id: i32,
        data: &UpdatePresetEvent,
    ) -> QueryResult<PresetEvent> {
        diesel::update(preset_event)
            .filter(id.eq(preset_event_id))
            .set(data)
            .get_result(conn)
    }

    /// Delete a `PresetEvent` given its ID.
    /// Returns the number of rows deleted (should be `1`).
    #[inline]
    pub fn delete(conn: &Database, preset_event_id: i32) -> QueryResult<usize> {
        diesel::delete(preset_event)
            .filter(id.eq(preset_event_id))
            .execute(conn)
    }
}
