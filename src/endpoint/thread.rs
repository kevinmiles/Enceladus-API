use crate::{
    controller::thread::{ExternalInsertThread, Thread, UpdateThread},
    endpoint::helpers::RocketResult,
    DataDB,
};
use rocket::{post, response::status::Created};
use rocket_contrib::json::Json;

generic_all!(Thread);
generic_get!(Thread);
generic_patch!(Thread);
generic_delete!(Thread);

// We can't use `generic_post!()` here because we need to use `ExternalInsertThread`
// as the parameter type.
#[inline]
#[post("/", data = "<data>")]
pub fn post(conn: DataDB, data: Json<ExternalInsertThread>) -> RocketResult<Created<Json<Thread>>> {
    created!(Thread::create(&conn, &data))
}
