use crate::{
    controller::{
        thread::{ExternalInsertThread, Thread, UpdateThread},
        user::User,
    },
    endpoint::helpers::RocketResult,
    DataDB,
};
use rocket::{delete, get, http::Status, patch, post, response::status::Created};
use rocket_contrib::json::{Json, JsonValue};
use std::collections::BTreeSet;

generic_all!(Thread);
generic_get!(Thread);

#[inline]
#[get("/<id>/full")]
pub fn get_full(conn: DataDB, id: i32) -> RocketResult<JsonValue> {
    Ok(Thread::find_id_with_foreign_keys(&conn, id)
        .map_err(crate::endpoint::helpers::error_mapper)?
        .into())
}

#[inline]
#[post("/", data = "<data>")]
pub fn post(
    conn: DataDB,
    user: User,
    data: Json<ExternalInsertThread>,
) -> RocketResult<Created<Json<Thread>>> {
    created!(Thread::create(&conn, &data, user.id))
}

#[inline]
#[patch("/<id>", data = "<data>")]
pub fn patch(
    conn: DataDB,
    user: User,
    id: i32,
    data: Json<UpdateThread>,
) -> RocketResult<Json<Thread>> {
    if !user.can_modify_thread(&conn, id) {
        return Err(Status::Unauthorized);
    }

    // Restrict changing `.sections_id` to reordering, not adding or removing.
    if data.sections_id.is_some() {
        let current_thread = Thread::find_id(&conn, id).unwrap();

        let current_sections: BTreeSet<_> = current_thread.sections_id.iter().collect();
        let proposed_sections: BTreeSet<_> = data.sections_id.as_ref().unwrap().iter().collect();

        if current_sections != proposed_sections {
            return Err(Status::PreconditionFailed);
        }
    }

    // Restrict changing `.events_id` to reordering, not adding or removing.
    if data.events_id.is_some() {
        let current_thread = Thread::find_id(&conn, id).unwrap();

        let current_events: BTreeSet<_> = current_thread.events_id.iter().collect();
        let proposed_events: BTreeSet<_> = data.events_id.as_ref().unwrap().iter().collect();

        if current_events != proposed_events {
            return Err(Status::PreconditionFailed);
        }
    }

    return json_result!(Thread::update(&conn, id, &data));
}

#[inline]
#[delete("/<id>")]
pub fn delete(conn: DataDB, user: User, id: i32) -> RocketResult<Status> {
    if user.can_modify_thread(&conn, id) {
        return no_content!(Thread::delete(&conn, id));
    }

    Err(Status::Unauthorized)
}
