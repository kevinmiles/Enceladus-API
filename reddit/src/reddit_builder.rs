use crate::{scope::Scope, Reddit};

pub struct RedditBuilder<'a> {
    redirect_uri: Option<&'a str>,
    user_agent:   Option<&'a str>,
    client_id:    Option<&'a str>,
    secret:       Option<&'a str>,
    permanent:    Option<bool>,
    scopes:       Option<&'a [Scope]>,
}

// This is explicitly _not_ an implementation of `std::default::Default`,
// as currently (2019-02-22), trait objects cannot be `const fn`.
impl RedditBuilder<'_> {
    #[inline(always)]
    pub const fn default() -> Self {
        RedditBuilder {
            redirect_uri: None,
            user_agent:   None,
            client_id:    None,
            secret:       None,
            permanent:    None,
            scopes:       None,
        }
    }
}

impl<'a> RedditBuilder<'a> {
    // FIXME Make this `const fn` when `.unwrap()` or similar becomes `const fn`.
    #[inline]
    pub fn build(self) -> Option<Reddit<'a>> {
        Some(Reddit {
            redirect_uri: self.redirect_uri?,
            user_agent:   self.user_agent?,
            client_id:    self.client_id?,
            secret:       self.secret?,
            permanent:    self.permanent?,
            scopes:       self.scopes?,
        })
    }

    #[inline(always)]
    pub const fn with_redirect_uri(mut self, redirect_uri: &'a str) -> Self {
        self.redirect_uri = Some(redirect_uri);
        self
    }

    #[inline(always)]
    pub const fn with_user_agent(mut self, user_agent: &'a str) -> Self {
        self.user_agent = Some(user_agent);
        self
    }

    #[inline(always)]
    pub const fn with_client_id(mut self, client_id: &'a str) -> Self {
        self.client_id = Some(client_id);
        self
    }

    #[inline(always)]
    pub const fn with_secret(mut self, secret: &'a str) -> Self {
        self.secret = Some(secret);
        self
    }

    #[inline(always)]
    pub const fn with_permanent(mut self, permanent: bool) -> Self {
        self.permanent = Some(permanent);
        self
    }

    #[inline(always)]
    pub const fn with_scopes(mut self, scopes: &'a [Scope]) -> Self {
        self.scopes = Some(scopes);
        self
    }
}
