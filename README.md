# Enceladus API

## Build process

In order to build the API,
you must have Rust installed.
If you do not already,
you can find the instructions on how to do so
[in the Rust book](https://doc.rust-lang.org/1.0.0/book/installing-rust.html).

To clone this repository,
run `git clone git@github.com:r-spacex/Enceladus-API-rs.git`.

Once inside the repository,
you can build a binary for development with `cargo build`,
which will have all user endpoints enabled.
For release (which has certain endpoints disabled and optimizations enabled),
run `cargo build --release`.
The resulting binaries will be `./target/debug/enceladus-api` and `./target/release/enceladus-api` respectively.

## Changes to database

If you're making a change to the database itself,
you'll likely want the Diesel CLI installed (`cargo install diesel`).
Look up Diesel's CLI syntax if you're not familiar.
The only thing of note is that, for this project,
it is preferred to use sequential IDs rather than the datetime.

After creating a migration, be sure to run `diesel migrate run`!
Without that, `src/schema.rs` will not be updated.
