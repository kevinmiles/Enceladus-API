#![allow(non_snake_case)]

use crate::{
    controller::{claim::Claim, thread::Thread},
    schema::user::{self, dsl::*},
    DataDB, Database,
};
use enceladus_macros::generate_structs;
use lazy_static::lazy_static;
use lru_cache::LruCache;
use parking_lot::Mutex;
use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
    Outcome,
};
use rocket_contrib::databases::diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

lazy_static! {
    /// A global cache, containing a mapping of IDs to their respective `Event`.
    ///
    /// The cache is protected by a `Mutex`,
    /// ensuring there is only ever at most one writer at a time.
    /// Note that even when reading,
    /// there must be a lock on mutability,
    /// as the `LruCache` must be able to update itself.
    ///
    /// To read from the cache,
    /// you'll want to call `CACHE.lock()` before performing normal operations.
    /// ```
    static ref CACHE: Mutex<LruCache<i32, User>> = Mutex::new(LruCache::new(100));
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
    /// Check if the user is an admin of a given subreddit.
    ///
    /// This is necessary, as Rust gives us no other way of
    /// accessing fields dynamically at runtime.
    #[inline]
    pub fn is_admin_of(&self, subreddit: Option<&str>) -> bool {
        match subreddit {
            Some("spacex") => self.spacex__is_admin,
            _ => false,
        }
    }

    /// Is the provided user able to modify data (including sections and events)
    /// on the indicated thread?
    ///
    /// Authentication levels (from low to high):
    ///
    /// - None
    /// - Logged in (everyday user)
    /// - Thread author
    /// - Subreddit admin
    /// - Global admin
    ///
    /// This function verifies that a user is, at a minimum, the thread author.
    #[inline]
    pub fn can_modify_thread(&self, conn: &DataDB, thread_id: i32) -> bool {
        // Global admins can change anything.
        if self.is_global_admin {
            return true;
        }

        let thread = {
            let thread = Thread::find_id(conn, thread_id);

            // The thread we want to add the event to doesn't exist.
            if thread.is_err() {
                return false;
            }

            thread.unwrap()
        };

        // The user is a local admin.
        if self.is_admin_of(thread.subreddit.as_ref().map(|s| s.as_str())) {
            return true;
        }

        // The user is the thread creator.
        thread.created_by_user_id == self.id
    }

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
        let mut cache = CACHE.lock();
        if cache.contains_key(&user_id) {
            Ok(cache.get_mut(&user_id).unwrap().clone())
        } else {
            let result: Self = user.find(user_id).first(conn)?;
            cache.insert(user_id, result.clone());
            Ok(result)
        }
    }

    /// Create a `User` given the data.
    ///
    /// The inserted row is added to the global cache and returned.
    #[inline]
    pub fn create(conn: &Database, data: &InsertUser) -> QueryResult<Self> {
        let result: Self = diesel::insert_into(user).values(data).get_result(conn)?;
        CACHE.lock().insert(result.id, result.clone());
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
        CACHE.lock().insert(result.id, result.clone());
        Ok(result)
    }

    /// Delete a `User` given its ID.
    ///
    /// Removes the entry from cache and returns the number of rows deleted (should be `1`).
    #[inline]
    pub fn delete(conn: &Database, user_id: i32) -> QueryResult<usize> {
        CACHE.lock().remove(&user_id);
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
