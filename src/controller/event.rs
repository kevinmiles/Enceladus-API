use crate::{
    schema::event::{self, dsl::*},
    Database,
};
use enceladus_macros::generate_structs;
use rocket_contrib::databases::diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

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
    #[inline]
    pub fn find_all(conn: &Database) -> QueryResult<Vec<Event>> {
        event.load(conn)
    }

    #[inline]
    pub fn find_id(conn: &Database, event_id: i32) -> QueryResult<Event> {
        event.find(event_id).first(conn)
    }

    #[inline]
    pub fn create(conn: &Database, data: &InsertEvent) -> QueryResult<Event> {
        diesel::insert_into(event)
            .values(data)
            .execute(conn)
            .map(|_| find_inserted!(event, conn))
    }

    #[inline]
    pub fn update(conn: &Database, event_id: i32, data: &UpdateEvent) -> QueryResult<Event> {
        diesel::update(event)
            .filter(id.eq(event_id))
            .set(data)
            .execute(conn)
            .map(|_| Event::find_id(conn, event_id).unwrap())
    }

    #[inline]
    pub fn delete(conn: &Database, event_id: i32) -> QueryResult<usize> {
        diesel::delete(event).filter(id.eq(event_id)).execute(conn)
    }
}
