use crate::{
    controller::{
        section::{InsertSection, Section, UpdateSection},
        user::User,
    },
    endpoint::helpers::RocketResult,
    DataDB,
};
use rocket::{delete, http::Status, patch, post, response::status::Created};
use rocket_contrib::json::Json;

generic_all!(Section);
generic_get!(Section);

#[inline]
#[post("/", data = "<data>")]
pub fn post(
    conn: DataDB,
    user: User,
    data: Json<InsertSection>,
) -> RocketResult<Created<Json<Section>>> {
    if user.can_modify_thread(&conn, data.in_thread_id) {
        return created!(Section::create(&conn, &data));
    }

    Err(Status::Unauthorized)
}

#[inline]
#[patch("/<id>", data = "<data>")]
pub fn patch(
    conn: DataDB,
    user: User,
    id: i32,
    data: Json<UpdateSection>,
) -> RocketResult<Json<Section>> {
    let section = Section::find_id(&conn, id);

    if section.is_ok() && user.can_modify_thread(&conn, section.unwrap().in_thread_id) {
        return json_result!(Section::update(&conn, id, &data));
    }

    Err(Status::Unauthorized)
}

#[inline]
#[delete("/<id>")]
pub fn delete(conn: DataDB, user: User, id: i32) -> RocketResult<Status> {
    let section = Section::find_id(&conn, id);

    if section.is_ok() && user.can_modify_thread(&conn, section.unwrap().in_thread_id) {
        return no_content!(Section::delete(&conn, id));
    }

    Err(Status::Unauthorized)
}
