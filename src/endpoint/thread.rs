use crate::{
    controller::{
        thread::{ExternalInsertThread, Thread, UpdateThread},
        user::User,
    },
    endpoint::helpers::RocketResult,
    DataDB,
};
use rocket::{delete, http::Status, patch, post, response::status::Created};
use rocket_contrib::json::Json;

generic_all!(Thread);
generic_get!(Thread);

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
    if user.can_modify_thread(&conn, id) {
        return json_result!(Thread::update(&conn, id, &data));
    }

    Err(Status::Unauthorized)
}

#[inline]
#[delete("/<id>")]
pub fn delete(conn: DataDB, user: User, id: i32) -> RocketResult<Status> {
    if user.can_modify_thread(&conn, id) {
        return no_content!(Thread::delete(&conn, id));
    }

    Err(Status::Unauthorized)
}
