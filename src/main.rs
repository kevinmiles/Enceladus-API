#![feature(
    proc_macro_hygiene,
    decl_macro,
    concat_idents,
    custom_attribute,
    const_str_as_bytes
)]
#![deny(clippy::all)]
#![allow(intra_doc_link_resolution_failure, clippy::match_bool)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate dotenv_codegen;

pub mod controller;
pub mod endpoint;
pub mod reddit;
pub mod schema;

#[cfg(test)]
mod tests;

use crate::endpoint::*;
use dotenv::dotenv;
use rocket::{routes, Rocket};
use rocket_contrib::{database, helmet::SpaceHelmet};
use tokio::{prelude::*, runtime::current_thread};

pub static mut TOKIO: Option<current_thread::Handle> = None;

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
        .mount("/meta", routes![meta::meta])
        .mount("/oauth", routes![oauth::oauth, oauth::callback])
        .mount(
            "/v1/user",
            #[cfg(test)]
            all_routes!(user),
            #[cfg(not(test))]
            routes![user::all, user::get],
        )
        .mount("/v1/preset_event", all_routes!(preset_event))
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
pub fn main() {
    let (tokio_tx, tokio_rx) = tokio::sync::oneshot::channel();
    let (handle_tx, handle_rx) = std::sync::mpsc::channel();

    let tokio_thread = std::thread::spawn(move || {
        let mut runtime = current_thread::Runtime::new().expect("Unable to create tokio runtime.");

        // Send the runtime handle to the receiver below,
        // where it is set to the global variable `TOKIO`.
        handle_tx
            .send(runtime.handle())
            .expect("Unable to provide tokio's runtime handle to receiver.");

        // Continue running until instructed to shut down.
        runtime
            .spawn({ tokio_rx.map_err(|err| panic!("Error on the shutdown channel: {:?}", err)) })
            .run()
            .expect("Tokio runtime execution failed.");
    });

    // Set the global `TOKIO`,
    // which is used to add `Future`s to the event loop.
    unsafe { TOKIO = Some(handle_rx.recv().unwrap()) };

    server().launch();

    // End tokio's runtime.
    tokio_tx.send(()).unwrap();
    tokio_thread.join().unwrap();
}
