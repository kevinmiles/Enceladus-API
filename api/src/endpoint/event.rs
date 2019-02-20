use crate::{
    controller::{Event, InsertEvent, Thread, UpdateEvent, User},
    endpoint::helpers::RocketResult,
    DataDB,
};
use rocket::{delete, http::Status, patch, post, response::status::Created};
use rocket_contrib::json::Json;

generic_all!(Event);
generic_get!(Event);

#[inline]
#[post("/", data = "<data>")]
pub fn post(
    conn: DataDB,
    user: User,
    data: Json<InsertEvent>,
) -> RocketResult<Created<Json<Event>>> {
    if user.can_modify_thread(&conn, data.in_thread_id) {
        let thread = Thread::find_id(&conn, data.in_thread_id).unwrap();

        // Ensure the provided columns are of the expected types and length.
        if !data.cols.is_array()
            || thread.event_column_headers.len() != data.cols.clone().as_array().unwrap().len()
            || !data
                .cols
                .clone()
                .as_array()
                .unwrap()
                .iter()
                .zip(0..)
                .all(|(val, i)| match thread.space__utc_col_index {
                    Some(n) if i == n => val.is_number(),
                    _ => val.is_string(),
                })
        {
            return Err(Status::UnprocessableEntity);
        }

        return created!(Event::create(&conn, &data));
    }

    Err(Status::Unauthorized)
}

#[inline]
#[patch("/<id>", data = "<data>")]
pub fn patch(
    conn: DataDB,
    user: User,
    id: i32,
    data: Json<UpdateEvent>,
) -> RocketResult<Json<Event>> {
    let event = Event::find_id(&conn, id);

    if event.is_ok() && user.can_modify_thread(&conn, event.unwrap().in_thread_id) {
        return json_result!(Event::update(&conn, id, &data));
    }

    Err(Status::Unauthorized)
}

#[inline]
#[delete("/<id>")]
pub fn delete(conn: DataDB, user: User, id: i32) -> RocketResult<Status> {
    let event = Event::find_id(&conn, id);

    if event.is_ok() && user.can_modify_thread(&conn, event.unwrap().in_thread_id) {
        return no_content!(Event::delete(&conn, id));
    }

    Err(Status::Unauthorized)
}
