use diesel::prelude::*;
use diesel_migrations::run_pending_migrations;
use dotenv::dotenv;
use std::{env, fs::create_dir};

fn run_migrations() {
    // If this fails, either the directory exists or the user lacks permission.
    // In the latter case, the user will get an error when trying to establish a connection.
    let _ = create_dir("databases");

    let db_url = env::var("DATABASE_URL").expect("environment variable DATABASE_URL must be set");

    run_pending_migrations(
        &PgConnection::establish(&db_url).expect("Error connecting to database"),
    )
    .expect("Error running migrations");
}

fn main() {
    dotenv().ok();

    run_migrations();
}
