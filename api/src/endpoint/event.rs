use crate::{
    controller::{Event, InsertEvent, Thread, UpdateEvent, User},
    endpoint::helpers::RocketResult,
    DataDB,
};
use rocket::{delete, http::Status, patch, post, response::status::Created};
use rocket_contrib::json::Json;

generic_all!(Event);
generic_get!(Event);

/// Create an `Event`.
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

        let ret_val = created!(Event::create(&conn, &data));

        thread.update_on_reddit(&conn).unwrap();

        return ret_val;
    }

    Err(Status::Unauthorized)
}

/// We need to define a type discriminant to allow Rocket to discern between
/// an update on all columns and an update on a specific column.
///
/// When updating all columns,
/// we're expecting a regular `UpdateEvent` object.
/// When updating a single column,
/// we're expecting an array containing the [key, new value].
#[derive(serde::Deserialize)]
#[serde(untagged)]
pub enum UpdateEventDiscriminant {
    FullEvent(UpdateEvent),
    PartialEvent(Vec<(usize, serde_json::Value)>),
}

/// Discriminate between the two types,
/// calling the `patch_full_event` method as necessary.
#[inline]
#[patch("/<id>", data = "<data>")]
pub fn patch(
    conn: DataDB,
    user: User,
    id: i32,
    data: Json<UpdateEventDiscriminant>,
) -> RocketResult<Json<Event>> {
    use UpdateEventDiscriminant::*;
    match data.into_inner() {
        FullEvent(data) => patch_full_event(conn, user, id, data),
        PartialEvent(data) => {
            if let Ok(mut event) = Event::find_id(&conn, id) {
                let event_fields = &mut event.cols;

                for (key, value) in data.into_iter() {
                    event_fields[key] = value;
                }

                patch_full_event(
                    conn,
                    user,
                    id,
                    UpdateEvent {
                        cols: Some(event.cols),
                        ..UpdateEvent::default()
                    },
                )
            } else {
                Err(Status::NotFound)
            }
        }
    }
}

/// Update the `Event` on Reddit and in the database.
#[inline]
pub fn patch_full_event(
    conn: DataDB,
    user: User,
    id: i32,
    data: UpdateEvent,
) -> RocketResult<Json<Event>> {
    if let Ok(event) = Event::find_id(&conn, id) {
        if user.can_modify_thread(&conn, event.in_thread_id) {
            let ret_val = json_result!(Event::update(&conn, id, &data));

            Thread::find_id(&conn, event.in_thread_id)
                .unwrap()
                .update_on_reddit(&conn)
                .unwrap();

            ret_val
        } else {
            Err(Status::Unauthorized)
        }
    } else {
        Err(Status::NotFound)
    }
}

/// Delete an `Event` as well as any references to its ID.
#[inline]
#[delete("/<id>")]
pub fn delete(conn: DataDB, user: User, id: i32) -> RocketResult<Status> {
    if let Ok(event) = Event::find_id(&conn, id) {
        if user.can_modify_thread(&conn, event.in_thread_id) {
            let ret_val = no_content!(Event::delete(&conn, id));

            Thread::find_id(&conn, event.in_thread_id)
                .unwrap()
                .update_on_reddit(&conn)
                .unwrap();

            return ret_val;
        }
    }

    Err(Status::Unauthorized)
}
