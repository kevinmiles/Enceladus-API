#![feature(proc_macro_hygiene, decl_macro, concat_idents)]
#![allow(proc_macro_derive_resolution_fallback)]
#![deny(warnings)]
#![deny(clippy::all)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

mod controller;
mod endpoint;
mod schema;

use crate::endpoint::*;
use rocket_contrib::helmet::SpaceHelmet;

#[database("data")]
pub struct DataDB(diesel::SqliteConnection);

fn main() {
    rocket::ignite()
        .attach(SpaceHelmet::default())
        .attach(DataDB::fairing())
        .mount(
            "/v1/user",
            if cfg!(debug_assertions) {
                routes![user::all, user::get, user::post, user::patch, user::delete]
            } else {
                routes![user::all, user::get]
            },
        )
        .launch();
}
