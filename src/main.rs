#![feature(proc_macro_hygiene, decl_macro, concat_idents, custom_attribute)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]
#![deny(warnings, clippy::all)]

#[macro_use]
extern crate diesel;

mod controller;
mod endpoint;
mod schema;

#[cfg(test)]
mod tests;

use crate::endpoint::*;
use rocket::{routes, Rocket};
use rocket_contrib::{database, helmet::SpaceHelmet};

// single point to change if we need to alter the DBMS
pub type Database = diesel::SqliteConnection;
#[database("data")]
pub struct DataDB(Database);

macro_rules! all_routes {
    ($ns:ident) => {
        routes![$ns::all, $ns::get, $ns::post, $ns::patch, $ns::delete]
    };
}

/// Creates a server,
/// attaching middleware for security and database access.
/// Routes are then mounted (some conditionally).
pub fn server() -> Rocket {
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
}

/// Launch the server.
fn main() {
    server().launch();
}
