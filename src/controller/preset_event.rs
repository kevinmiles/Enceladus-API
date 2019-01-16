use crate::{
    schema::{preset_event, preset_event::dsl::*},
    Database,
};
use enceladus_macros::{InsertStruct, UpdateStruct};
use rocket_contrib::databases::diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

#[derive(Serialize, Deserialize, Queryable, Clone, InsertStruct, UpdateStruct)]
#[table_name = "preset_event"]
#[serde(deny_unknown_fields)]
pub struct PresetEvent {
    #[no_update]
    pub id: i32,
    #[insert_default]
    pub holds_clock: bool,
    pub message: String,
    pub name: String,
}

impl PresetEvent {
    #[inline]
    pub fn find_all(conn: &Database) -> QueryResult<Vec<PresetEvent>> {
        preset_event.load(conn)
    }

    #[inline]
    pub fn find_id(conn: &Database, preset_event_id: i32) -> QueryResult<PresetEvent> {
        preset_event.find(preset_event_id).first(conn)
    }

    #[inline]
    pub fn create(conn: &Database, data: &InsertPresetEvent) -> QueryResult<PresetEvent> {
        diesel::insert_into(preset_event)
            .values(data)
            .execute(conn)
            .map(|_| find_inserted!(preset_event, conn))
    }

    #[inline]
    pub fn update(
        conn: &Database,
        preset_event_id: i32,
        data: &UpdatePresetEvent,
    ) -> QueryResult<PresetEvent> {
        diesel::update(preset_event)
            .filter(id.eq(preset_event_id))
            .set(data)
            .execute(conn)
            .map(|_| PresetEvent::find_id(conn, preset_event_id).unwrap())
    }

    #[inline]
    pub fn delete(conn: &Database, preset_event_id: i32) -> QueryResult<usize> {
        diesel::delete(preset_event)
            .filter(id.eq(preset_event_id))
            .execute(conn)
    }
}
