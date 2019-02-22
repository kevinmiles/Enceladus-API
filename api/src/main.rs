#![feature(proc_macro_hygiene, decl_macro, custom_attribute, const_str_as_bytes)]
#![deny(clippy::all)]
#![warn(clippy::nursery)] // Don't deny, as there may be unknown bugs.
#![allow(intra_doc_link_resolution_failure, clippy::match_bool)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate dotenv_codegen;

mod controller;
mod endpoint;
mod fairing;
mod reddit;
mod schema;

#[cfg(test)]
mod tests;

use dotenv::dotenv;
use endpoint::*;
use fairing::*;
use rocket::{routes, Rocket};
use rocket_contrib::{database, helmet::SpaceHelmet};

/// Single point to change if we need to alter the DBMS.
pub type Database = diesel::PgConnection;
#[database("data")]
pub struct DataDB(Database);

macro_rules! all_routes {
    ($ns:ident) => {
        routes![$ns::all, $ns::get, $ns::post, $ns::patch, $ns::delete]
    };
}

/// Returns a globally unique identifier.
/// Specifically, v4, which is not based on any input factors.
#[inline]
pub fn guid() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Creates a server,
/// attaching middleware for security and database access.
/// Routes are then mounted (some conditionally).
#[inline]
pub fn server() -> Rocket {
    // Although we inline most variables at compile-time,
    // Rocket doesn't let us do this.
    // As such, we have to load the `.env` file at runtime as well.
    // **Do not** remove it from the build script,
    // as we still want to inline wherever possible.
    dotenv().ok();

    rocket::ignite()
        .attach(SpaceHelmet::default())
        .attach(DataDB::fairing())
        .attach(FeatureFilter::fairing())
        .mount("/meta", routes![meta::meta])
        .mount("/oauth", routes![oauth::oauth, oauth::callback])
        .mount(
            "/v1/user",
            #[cfg(test)]
            all_routes!(user),
            #[cfg(not(test))]
            routes![user::all, user::get],
        )
        .mount(
            "/v1/thread",
            routes![
                thread::all,
                thread::get,
                thread::get_full,
                thread::post,
                thread::patch,
                thread::delete,
            ],
        )
        .mount("/v1/section", all_routes!(section))
        .mount("/v1/event", all_routes!(event))
}

/// Launch the server.
/// Uses the port number defined in the environment variable `ROCKET_PORT`.
/// If not defined, defaults to `8000`.
fn main() {
    server().launch();
}
