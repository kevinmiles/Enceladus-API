#![allow(non_snake_case)]

use crate::{
    controller::claim::Claim,
    schema::user::{self, dsl::*},
    DataDB, Database,
};
use enceladus_macros::generate_structs;
use hashbrown::HashMap;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
    Outcome,
};
use rocket_contrib::databases::diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

lazy_static! {
    /// A global cache, containing a mapping of IDs to their respective `User`.
    ///
    /// The cache is protected by a `RwLock`,
    /// ensuring there is only ever at most one writer (and no readers) at a point in time.
    ///
    /// To read from the cache,
    /// you'll want to call `CACHE.read()` before performing normal operations.
    /// The same is true for `CACHE.write()`.
    ///
    /// It is _highly_ recommended to manually call `drop()` after you're done using the lock.
    /// This ensures that nothing else is blocked from accessing the cache if necessary.
    ///
    /// Here's example of when this is necessary to ensure working code:
    ///
    /// ```rust
    /// // Obtain a read lock on the global cache.
    /// let cache = CACHE.read();
    ///
    /// if cache.contains_key("foo") {
    ///     // Do something with the value.
    ///     cache["foo"]
    /// } else {
    ///     // Manually drop the `cache` variable,
    ///     // letting us obtain a write lock.
    ///     std::mem::drop(cache);
    ///
    ///     // Now we can obtain a write lock without having to wait
    ///     // for the read lock to be dropped automatically.
    ///     // Note that this _would not happen_ until _after_ the request for the write lock,
    ///     // causing a deadlock in the code not caught by the compiler.
    ///     CACHE.write().insert("foo", "bar");
    /// }
    /// ```
    static ref CACHE: RwLock<HashMap<i32, User>> = RwLock::new(HashMap::new());
}

generate_structs! {
    User("user") {
        auto id: i32,
        readonly reddit_username: String,
        lang: String = "en",
        private refresh_token: String,
        is_global_admin: bool = false,
        spacex__is_admin: bool = false,
        spacex__is_mod: bool = false,
        spacex__is_slack_member: bool = false,
    }
}

impl User {
    /// Find all `User`s in the database.
    ///
    /// Does _not_ use cache (reading or writing),
    /// so as to avoid storing values rarely accessed.
    #[inline]
    pub fn find_all(conn: &Database) -> QueryResult<Vec<Self>> {
        user.load(conn)
    }

    /// Find a specific `User` given its ID.
    ///
    /// Internally uses a cache to limit database accesses.
    #[inline]
    pub fn find_id(conn: &Database, user_id: i32) -> QueryResult<Self> {
        let cache = CACHE.read();
        if cache.contains_key(&user_id) {
            Ok(cache[&user_id].clone())
        } else {
            // drop the read lock on the cache,
            // ensuring we can call `CACHE.write()` without issue
            std::mem::drop(cache);

            let result: Self = user.find(user_id).first(conn)?;
            CACHE.write().insert(user_id, result.clone());
            Ok(result)
        }
    }

    /// Create a `User` given the data.
    ///
    /// The inserted row is added to the global cache and returned.
    #[inline]
    pub fn create(conn: &Database, data: &InsertUser) -> QueryResult<Self> {
        let result: Self = diesel::insert_into(user).values(data).get_result(conn)?;
        CACHE.write().insert(result.id, result.clone());
        Ok(result)
    }

    /// Update a `User` given an ID and the data to update.
    ///
    /// The entry is updated in the database, added to cache, and returned.
    #[inline]
    pub fn update(conn: &Database, user_id: i32, data: &UpdateUser) -> QueryResult<Self> {
        let result: Self = diesel::update(user)
            .filter(id.eq(user_id))
            .set(data)
            .get_result(conn)?;
        CACHE.write().insert(result.id, result.clone());
        Ok(result)
    }

    /// Delete a `User` given its ID.
    ///
    /// Removes the entry from cache and returns the number of rows deleted (should be `1`).
    #[inline]
    pub fn delete(conn: &Database, user_id: i32) -> QueryResult<usize> {
        CACHE.write().remove(&user_id);
        diesel::delete(user).filter(id.eq(user_id)).execute(conn)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = &'a str;

    #[inline]
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let header = request.headers().get_one("Authentication");
        if header.is_none() {
            return Outcome::Failure((
                Status::Unauthorized,
                r#"Expected "Authentication" header to be present"#,
            ));
        }

        let header_contents = header.unwrap();
        if !header_contents.starts_with("bearer ") && !header_contents.starts_with("Bearer ") {
            return Outcome::Failure((
                Status::BadRequest,
                r#"Expected "Authentication" header to begin with "bearer " or "Bearer ""#,
            ));
        }

        let user_id = Claim::get_user_id(&header_contents[7..]);
        if user_id.is_err() {
            return Outcome::Failure((
                Status::BadRequest,
                r#""Authentication" header cannot be decoded"#,
            ));
        }

        let database = request
            .guard::<DataDB>()
            .succeeded()
            .expect("Unable to access database");

        match Self::find_id(&database, user_id.unwrap()) {
            Ok(authenticated_user) => Outcome::Success(authenticated_user),
            Err(_) => Outcome::Failure((Status::BadRequest, "Unable to find user")),
        }
    }
}
