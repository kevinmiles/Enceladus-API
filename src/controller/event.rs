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
    #[inline]
    pub fn find_all(conn: &Database) -> QueryResult<Vec<Event>> {
        event.load(conn)
    }

    #[inline]
    pub fn find_id(conn: &Database, event_id: i32) -> QueryResult<Event> {
        let cache = CACHE.read();
        if cache.contains_key(&event_id) {
            Ok(cache[&event_id].clone())
        } else {
            // drop the read lock on the cache,
            // ensuring we can call `CACHE.write()` without issue
            std::mem::drop(cache);

            let result: Event = event.find(event_id).first(conn)?;
            CACHE.write().insert(event_id, result.clone());
            Ok(result)
        }
    }

    #[inline]
    pub fn create(conn: &Database, data: &InsertEvent) -> QueryResult<Event> {
        let result: Event = diesel::insert_into(event).values(data).get_result(conn)?;
        CACHE.write().insert(result.id, result.clone());
        Ok(result)
    }

    #[inline]
    pub fn update(conn: &Database, event_id: i32, data: &UpdateEvent) -> QueryResult<Event> {
        let result: Event = diesel::update(event)
            .filter(id.eq(event_id))
            .set(data)
            .get_result(conn)?;
        CACHE.write().insert(result.id, result.clone());
        Ok(result)
    }

    #[inline]
    pub fn delete(conn: &Database, event_id: i32) -> QueryResult<usize> {
        CACHE.write().remove(&event_id);
        diesel::delete(event).filter(id.eq(event_id)).execute(conn)
    }
}
