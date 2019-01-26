use crate::{
    schema::section::{self, dsl::*},
    Database,
};
use enceladus_macros::generate_structs;
use rocket_contrib::databases::diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

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
        section.find(section_id).first(conn)
    }

    #[inline]
    pub fn create(conn: &Database, data: &InsertSection) -> QueryResult<Section> {
        diesel::insert_into(section)
            .values(data)
            .execute(conn)
            .map(|_| find_inserted!(section, conn))
    }

    #[inline]
    pub fn update(conn: &Database, section_id: i32, data: &UpdateSection) -> QueryResult<Section> {
        diesel::update(section)
            .filter(id.eq(section_id))
            .set(data)
            .execute(conn)
            .map(|_| Section::find_id(conn, section_id).unwrap())
    }

    #[inline]
    pub fn delete(conn: &Database, section_id: i32) -> QueryResult<usize> {
        diesel::delete(section)
            .filter(id.eq(section_id))
            .execute(conn)
    }
}
