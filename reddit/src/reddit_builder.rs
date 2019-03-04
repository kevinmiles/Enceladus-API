use crate::{scope::Scope, Reddit};

/// An implementation of the builder pattern for the `Reddit` struct.
pub struct RedditBuilder<'a> {
    redirect_uri: Option<&'a str>,
    user_agent:   Option<&'a str>,
    client_id:    Option<&'a str>,
    secret:       Option<&'a str>,
    permanent:    Option<bool>,
    scopes:       Option<&'a [Scope]>,
}

// This is explicitly _not_ an implementation of `std::default::Default`,
// as trait methods cannot currently be `const fn`.
impl RedditBuilder<'_> {
    /// The default `RedditBuilder`.
    /// All fields are `None`.
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
    /// Convert the builder into `Reddit`.
    /// Returns `Some` if there were no issues,
    /// `None` if some field could not be converted.
    #[inline]
    pub fn build(self) -> Option<Reddit<'a>> {
        Reddit {
            redirect_uri: self.redirect_uri?,
            user_agent:   self.user_agent?,
            client_id:    self.client_id?,
            secret:       self.secret?,
            permanent:    self.permanent?,
            scopes:       self.scopes?,
        }
        .into()
    }

    /// Set the redirect URI.
    #[inline(always)]
    pub const fn with_redirect_uri(mut self, redirect_uri: &'a str) -> Self {
        self.redirect_uri = Some(redirect_uri);
        self
    }

    /// Set the user agent send with each request.
    #[inline(always)]
    pub const fn with_user_agent(mut self, user_agent: &'a str) -> Self {
        self.user_agent = Some(user_agent);
        self
    }

    /// Set the client ID provided by Reddit.
    #[inline(always)]
    pub const fn with_client_id(mut self, client_id: &'a str) -> Self {
        self.client_id = Some(client_id);
        self
    }

    /// Set the client secret provided by Reddit.
    #[inline(always)]
    pub const fn with_secret(mut self, secret: &'a str) -> Self {
        self.secret = Some(secret);
        self
    }

    /// Set whether authorization should be temporary or permanent.
    #[inline(always)]
    pub const fn with_permanent(mut self, permanent: bool) -> Self {
        self.permanent = Some(permanent);
        self
    }

    /// Set the requested scopes.
    #[inline(always)]
    pub const fn with_scopes(mut self, scopes: &'a [Scope]) -> Self {
        self.scopes = Some(scopes);
        self
    }
}
