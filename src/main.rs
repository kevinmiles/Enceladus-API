#![feature(proc_macro_hygiene, decl_macro, concat_idents, custom_attribute)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]
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

// single point to change if we need to alter the DBMS
pub type Database = diesel::SqliteConnection;
#[database("data")]
pub struct DataDB(Database);

macro_rules! all_routes {
    ($ns:ident) => {
        routes![$ns::all, $ns::get, $ns::post, $ns::patch, $ns::delete]
    };
}

fn main() {
    rocket::ignite()
        .attach(SpaceHelmet::default())
        .attach(DataDB::fairing())
        .mount(
            "/v1/user",
            if cfg!(debug_assertions) {
                all_routes!(user)
            } else {
                routes![user::all, user::get]
            },
        )
        .mount("/v1/preset_event", all_routes!(preset_event))
        .launch();
}
