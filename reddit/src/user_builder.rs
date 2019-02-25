use crate::{Reddit, User};
use std::time::Instant;

pub struct UserBuilder<'a> {
    reddit_instance: Option<&'a Reddit<'a>>,
    refresh_token: Option<String>,
    #[allow(clippy::option_option)]
    access_token: Option<Option<String>>,
    expires_at: Option<Instant>,
}

// This is explicitly _not_ an implementation of `std::default::Default`,
// as trait objects cannot currently be `const fn`.
impl UserBuilder<'_> {
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
    // FIXME make this `const fn` when `.unwrap()` or similar becomes `const fn`.
    #[inline]
    pub fn build(self) -> Option<User<'a>> {
        Some(User {
            reddit_instance: self.reddit_instance?,
            refresh_token:   self.refresh_token?,
            access_token:    self.access_token?,
            expires_at:      self.expires_at?,
        })
    }

    #[inline(always)]
    pub const fn with_reddit_instance(mut self, reddit_instance: &'a Reddit<'a>) -> Self {
        self.reddit_instance = Some(reddit_instance);
        self
    }

    // FIXME make this `const fn` when `Option<String>` is allowed
    #[inline(always)]
    pub fn with_refresh_token(mut self, refresh_token: String) -> Self {
        self.refresh_token = Some(refresh_token);
        self
    }

    // FIXME make this `const fn` when `Option<String>` is allowed
    #[inline(always)]
    pub fn with_access_token(mut self, access_token: Option<String>) -> Self {
        self.access_token = Some(access_token);
        self
    }

    #[inline(always)]
    pub const fn with_expires_at(mut self, expires_at: Instant) -> Self {
        self.expires_at = Some(expires_at);
        self
    }
}
