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

#[inline]
#[post("/", data = "<data>")]
pub fn post(conn: DataDB, data: Json<ExternalInsertThread>) -> RocketResult<Created<Json<Thread>>> {
    created!(Thread::create(&conn, &data))
}