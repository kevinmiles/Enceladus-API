use crate::{
    controller::{
        global_admin::GlobalAdmin,
        preset_event::{InsertPresetEvent, PresetEvent, UpdatePresetEvent},
    },
    endpoint::helpers::RocketResult,
    DataDB,
};
use rocket::{delete, http::Status, patch, post, response::status::Created};
use rocket_contrib::json::Json;

generic_all!(PresetEvent);
generic_get!(PresetEvent);

#[inline]
#[post("/", data = "<data>")]
pub fn post(
    conn: DataDB,
    _admin: GlobalAdmin,
    data: Json<InsertPresetEvent>,
) -> RocketResult<Created<Json<PresetEvent>>> {
    created!(PresetEvent::create(&conn, &data))
}

#[inline]
#[patch("/<id>", data = "<data>")]
pub fn patch(
    conn: DataDB,
    _admin: GlobalAdmin,
    id: i32,
    data: Json<UpdatePresetEvent>,
) -> RocketResult<Json<PresetEvent>> {
    json_result!(PresetEvent::update(&conn, id, &data))
}

#[inline]
#[delete("/<id>")]
pub fn delete(conn: DataDB, _admin: GlobalAdmin, id: i32) -> RocketResult<Status> {
    no_content!(PresetEvent::delete(&conn, id))
}
