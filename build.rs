use diesel::prelude::*;
use diesel_migrations::run_pending_migrations;
use std::fs::create_dir;

fn run_migrations() {
    // If this fails, either the directory exists or the user lacks permission.
    // In the latter case, the user will get an error when trying to establish a connection.
    let _ = create_dir("databases");

    run_pending_migrations(
        &SqliteConnection::establish("databases/data.db").expect("Error connecting to database"),
    )
    .expect("Error running migrations");
}

fn main() {
    run_migrations();
}
