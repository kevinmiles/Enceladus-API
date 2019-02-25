#![deny(clippy::all)]
#![warn(clippy::nursery)] // Don't deny, as there may be unknown bugs.
#![allow(intra_doc_link_resolution_failure, clippy::match_bool)]

mod reddit_builder;
mod scope;
mod user_builder;

use itertools::Itertools;
use lazy_static::lazy_static;
use reddit_builder::RedditBuilder;
use reqwest::{header::USER_AGENT, Client, Url, UrlError};
pub use scope::Scope;
use serde::Deserialize;
use std::time::{Duration, Instant};
use user_builder::UserBuilder;

lazy_static! {
    static ref CLIENT: Client = Client::builder().gzip(true).build().unwrap();
}

/// Returns a globally unique identifier.
/// Specifically, v4, which is not based on any input factors.
#[inline]
pub fn guid() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub struct Reddit<'a> {
    redirect_uri: &'a str,
    user_agent:   &'a str,
    client_id:    &'a str,
    secret:       &'a str,
    permanent:    bool,
    scopes:       &'a [Scope],
}

pub struct User<'a> {
    reddit_instance: &'a Reddit<'a>,
    refresh_token:   String,
    access_token:    Option<String>,
    expires_at:      Instant,
}

/// Builder and getters
impl Reddit<'_> {
    #[inline(always)]
    pub const fn builder() -> RedditBuilder<'static> {
        RedditBuilder::default()
    }

    #[inline(always)]
    pub const fn redirect_uri(&self) -> &str {
        self.redirect_uri
    }

    #[inline(always)]
    pub const fn user_agent(&self) -> &str {
        self.user_agent
    }

    #[inline(always)]
    pub const fn client_id(&self) -> &str {
        self.client_id
    }

    #[inline(always)]
    pub const fn secret(&self) -> &str {
        self.secret
    }

    #[inline(always)]
    pub const fn permanent(&self) -> bool {
        self.permanent
    }

    #[inline(always)]
    pub const fn scopes(&self) -> &[Scope] {
        self.scopes
    }
}

// Getters
impl User<'_> {
    #[inline(always)]
    pub const fn builder() -> UserBuilder<'static> {
        UserBuilder::default()
    }

    #[inline(always)]
    pub const fn refresh_token(&self) -> &String {
        &self.refresh_token
    }

    #[inline(always)]
    pub const fn access_token(&self) -> &Option<String> {
        &self.access_token
    }

    #[inline(always)]
    pub const fn expires_at(&self) -> &Instant {
        &self.expires_at
    }
}

impl<'a> Reddit<'a> {
    #[inline]
    pub fn get_auth_url(&self, callback: &str) -> Result<String, UrlError> {
        Ok(Url::parse_with_params(
            "https://ssl.reddit.com/api/v1/authorize",
            &[
                ("response_type", "code"),
                ("client_id", self.client_id),
                ("state", callback),
                ("redirect_uri", self.redirect_uri),
                (
                    "duration",
                    match self.permanent {
                        true => "permanent",
                        false => "temporary",
                    },
                ),
                ("scope", &self.scopes().iter().join(" ")),
            ],
        )?
        .into_string())
    }

    #[inline]
    pub fn obtain_refresh_token(&self, code: &str) -> Result<User, reqwest::Error> {
        #[derive(Deserialize)]
        struct APIReturnType {
            access_token:  Option<String>,
            expires_in:    u64,
            refresh_token: Option<String>,
        }

        let data: APIReturnType = CLIENT
            .post("https://ssl.reddit.com/api/v1/access_token")
            .basic_auth(self.client_id, Some(self.secret))
            .form(&[
                ("grant_type", "authorization_code"),
                ("code", code),
                ("redirect_uri", self.redirect_uri),
            ])
            .send()?
            .json()?;

        Ok(User {
            reddit_instance: self,
            refresh_token:   data.refresh_token.unwrap(),
            access_token:    data.access_token,
            expires_at:      Instant::now() + Duration::from_secs(data.expires_in),
        })
    }
}

#[inline(always)]
fn endpoint(path: &str) -> String {
    format!("https://oauth.reddit.com{}", path)
}

/// Endpoints
impl User<'_> {
    #[inline]
    pub fn me(&self) -> Result<reqwest::Response, reqwest::Error> {
        CLIENT
            .get(&endpoint("/api/v1/me"))
            .header(USER_AGENT, self.reddit_instance.user_agent)
            .bearer_auth(&self.access_token.clone().unwrap())
            .send()
    }

    #[inline]
    pub fn prefs(&self) -> Result<reqwest::Response, reqwest::Error> {
        CLIENT
            .get(&endpoint("/api/v1/me/prefs"))
            .header(USER_AGENT, self.reddit_instance.user_agent)
            .bearer_auth(&self.access_token.clone().unwrap())
            .send()
    }
}

/// Methods that use endpoints
impl User<'_> {
    #[inline]
    pub fn username(&self) -> Result<String, reqwest::Error> {
        // We may use the `is_mod` field in the future
        // to automatically determine if the user is a moderator
        // of a specific subreddit
        #[allow(unused)]
        #[derive(Deserialize)]
        struct APIReturnType {
            name:   String,
            is_mod: bool,
        }

        Ok(self.me()?.json::<APIReturnType>()?.name)
    }

    #[inline]
    pub fn lang(&self) -> Result<String, reqwest::Error> {
        #[derive(Deserialize)]
        struct APIReturnType {
            lang: String,
        }

        Ok(self.prefs()?.json::<APIReturnType>()?.lang)
    }

    // TODO
    // submit self post
    // edit comment or self post
    // approve a submission or comment
    // set/unset sticky
}
