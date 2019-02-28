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
use std::time::{Duration, SystemTime};
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
    access_token:    String,
    expires_at:      SystemTime,
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

    /// Get the access token of the user,
    /// fetching a new one from Reddit if necessary.
    #[inline]
    pub fn access_token(&mut self) -> &String {
        // Refresh the access token if it's expired.
        if self.expires_at < SystemTime::now() {
            #[derive(Deserialize)]
            struct APIReturnType {
                access_token: String,
                expires_in:   u64,
            }

            let response: APIReturnType = CLIENT
                .post("https://ssl.reddit.com/api/v1/access_token")
                .basic_auth(
                    self.reddit_instance.client_id,
                    Some(self.reddit_instance.secret),
                )
                .form(&[
                    ("grant_type", "refresh_token"),
                    ("refresh_token", &self.refresh_token),
                ])
                .send()
                .unwrap()
                .json()
                .unwrap();

            self.access_token = response.access_token;
            self.expires_at = SystemTime::now() + Duration::from_secs(response.expires_in);
        }

        &self.access_token
    }

    #[inline(always)]
    pub const fn expires_at(&self) -> &SystemTime {
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
            access_token:  String,
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
            expires_at:      SystemTime::now() + Duration::from_secs(data.expires_in),
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
    pub fn me(&mut self) -> Result<reqwest::Response, reqwest::Error> {
        CLIENT
            .get(&endpoint("/api/v1/me"))
            .header(USER_AGENT, self.reddit_instance.user_agent)
            .bearer_auth(self.access_token())
            .send()
    }

    #[inline]
    pub fn prefs(&mut self) -> Result<reqwest::Response, reqwest::Error> {
        CLIENT
            .get(&endpoint("/api/v1/me/prefs"))
            .header(USER_AGENT, self.reddit_instance.user_agent)
            .bearer_auth(self.access_token())
            .send()
    }
}

/// Methods that use endpoints
impl User<'_> {
    #[inline]
    pub fn username(&mut self) -> Result<String, reqwest::Error> {
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
    pub fn lang(&mut self) -> Result<String, reqwest::Error> {
        #[derive(Deserialize)]
        struct APIReturnType {
            lang: String,
        }

        Ok(self.prefs()?.json::<APIReturnType>()?.lang)
    }

    #[inline]
    pub fn submit_self_post(
        &mut self,
        subreddit: &str,
        title: &str,
        text: Option<&str>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        CLIENT
            .post(&endpoint("/api/submit"))
            .header(USER_AGENT, self.reddit_instance.user_agent)
            .bearer_auth(self.access_token())
            .form(&[
                ("kind", "self"),
                ("api_type", "json"),
                ("extensions", "json"),
                ("sendreplies", "false"),
                ("sr", subreddit),
                ("title", title),
                ("text", text.unwrap_or_default()),
            ])
            .send()
    }

    // TODO
    // edit self post
    // approve submission
    // set sticky
}
