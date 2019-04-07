use crate::{Reddit, User};
use std::time::SystemTime;

/// An implementation of the builder pattern for the `User` struct.
#[derive(Debug)]
pub struct UserBuilder<'a> {
    reddit_instance: Option<&'a Reddit<'a>>,
    refresh_token:   Option<String>,
    access_token:    Option<String>,
    expires_at:      Option<SystemTime>,
}

// This is explicitly _not_ an implementation of `std::default::Default`,
// as trait methods cannot currently be `const fn`.
impl UserBuilder<'_> {
    /// The default `UserBuilder`.
    /// All fields are `None`.
    #[inline(always)]
    pub const fn default() -> Self {
        UserBuilder {
            reddit_instance: None,
            refresh_token:   None,
            access_token:    None,
            expires_at:      None,
        }
    }
}

impl<'a> UserBuilder<'a> {
    /// Convert the builder into `User`.
    /// Returns `Some` if there were no issues,
    /// `None` if some field could not be converted.
    #[inline]
    pub fn build(self) -> Option<User<'a>> {
        Some(User {
            reddit_instance: self.reddit_instance?,
            refresh_token:   self.refresh_token?,
            access_token:    self.access_token?,
            expires_at:      self.expires_at?,
        })
    }

    /// Set the parent instance of `Reddit`.
    #[inline(always)]
    pub const fn with_reddit_instance(mut self, reddit_instance: &'a Reddit<'a>) -> Self {
        self.reddit_instance = Some(reddit_instance);
        self
    }

    /// Set the user's refresh token.
    #[inline(always)]
    pub fn with_refresh_token(mut self, refresh_token: String) -> Self {
        self.refresh_token = Some(refresh_token);
        self
    }

    /// Set the user's access token.
    #[inline(always)]
    pub fn with_access_token(mut self, access_token: String) -> Self {
        self.access_token = Some(access_token);
        self
    }

    /// Set when the user's access token expires.
    #[inline(always)]
    pub const fn with_expires_at(mut self, expires_at: SystemTime) -> Self {
        self.expires_at = Some(expires_at);
        self
    }
}
