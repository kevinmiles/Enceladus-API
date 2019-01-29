use crate::{
    schema::section::{self, dsl::*},
    Database,
};
use enceladus_macros::generate_structs;
use hashbrown::HashMap;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use rocket_contrib::databases::diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

lazy_static! {
    static ref CACHE: RwLock<HashMap<i32, Section>> = RwLock::new(HashMap::new());
}

generate_structs! {
    Section("section") {
        auto id: i32,
        readonly is_events_section: bool = false,
        name: String = "",
        content: String = "",
        lock_held_by_user_id: Option<i32>,
        readonly in_thread_id: i32,
    }
}

impl Section {
    #[inline]
    pub fn find_all(conn: &Database) -> QueryResult<Vec<Section>> {
        section.load(conn)
    }

    #[inline]
    pub fn find_id(conn: &Database, section_id: i32) -> QueryResult<Section> {
        let cache = CACHE.read();
        if cache.contains_key(&section_id) {
            Ok(cache[&section_id].clone())
        } else {
            let result: Section = section.find(section_id).first(conn)?;
            CACHE.write().insert(section_id, result.clone());
            Ok(result)
        }
    }

    #[inline]
    pub fn create(conn: &Database, data: &InsertSection) -> QueryResult<Section> {
        let result: Section = diesel::insert_into(section).values(data).get_result(conn)?;
        CACHE.write().insert(result.id, result.clone());
        Ok(result)
    }

    #[inline]
    pub fn update(conn: &Database, section_id: i32, data: &UpdateSection) -> QueryResult<Section> {
        let result: Section = diesel::update(section)
            .filter(id.eq(section_id))
            .set(data)
            .get_result(conn)?;
        CACHE.write().insert(result.id, result.clone());
        Ok(result)
    }

    #[inline]
    pub fn delete(conn: &Database, section_id: i32) -> QueryResult<usize> {
        CACHE.write().remove(&section_id);
        diesel::delete(section)
            .filter(id.eq(section_id))
            .execute(conn)
    }
}
