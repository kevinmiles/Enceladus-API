use diesel::prelude::*;
use diesel_migrations::run_pending_migrations;
use dotenv::dotenv;
use std::env;

fn run_migrations() {
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
