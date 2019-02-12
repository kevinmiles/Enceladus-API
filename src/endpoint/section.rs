use crate::{
    controller::{
        section::{InsertSection, LockSection, Section, UpdateSection},
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

// We need to define a type discriminant to allow Rocket to discern between
// an update on the lock and an update on everything else.
// Rather than checking the existence of a field,
// we can rely on Serde to do that for us.
// As a bonus, it's future proof if we need to add additional fields.
#[derive(serde::Deserialize)]
#[serde(untagged)]
pub enum UpdateSectionDiscriminant {
    LockSection(LockSection),
    UpdateSection(UpdateSection),
}

// Discriminate between the two types,
// calling the appropriate method as necessary.
#[inline]
#[patch("/<id>", data = "<data>")]
pub fn patch(
    conn: DataDB,
    user: User,
    id: i32,
    data: Json<UpdateSectionDiscriminant>,
) -> RocketResult<Json<Section>> {
    use UpdateSectionDiscriminant::*;
    match data.into_inner() {
        LockSection(data) => set_lock(conn, user, id, data),
        UpdateSection(data) => update_fields(conn, user, id, data),
    }
}

#[inline]
fn set_lock(conn: DataDB, user: User, id: i32, data: LockSection) -> RocketResult<Json<Section>> {
    let section = Section::find_id(&conn, id);

    // The section we're trying to find wasn't found.
    if section.is_err() {
        return Err(Status::NotFound);
    }

    let section = section.unwrap();

    // Ensure the user possesses the authority to modify the lock if able to.
    if !user.can_modify_thread(&conn, section.in_thread_id) {
        return Err(Status::Unauthorized);
    }

    // (1) Let the user assign the (currently null) lock to themselves.
    // (2) Let the user revoke their own lock.
    if (section.lock_held_by_user_id.is_none() && data.lock_held_by_user_id == Some(user.id))
        || (section.lock_held_by_user_id == Some(user.id) && data.lock_held_by_user_id.is_none())
    {
        return json_result!(Section::set_lock(&conn, id, &data));
    }

    // The user isn't setting the lock to themselves,
    // or they possess the lock and are trying to set it to another user.
    Err(Status::Forbidden)
}

#[inline]
fn update_fields(
    conn: DataDB,
    user: User,
    id: i32,
    data: UpdateSection,
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
