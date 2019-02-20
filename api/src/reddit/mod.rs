mod scope;
pub use scope::Scope;

use crate::guid;
use itertools::Itertools;
use lazy_static::lazy_static;
use request::{header, Client, Url};
use reqwest as request;
use serde::Deserialize;
use std::{
    error::Error,
    time::{Duration, Instant},
};

type ResponseResult = Result<request::Response, request::Error>;

lazy_static! {
    static ref CLIENT: Client = {
        Client::builder()
            .gzip(true)
            .default_headers({
                let mut headers = header::HeaderMap::default();
                headers.insert(
                    header::USER_AGENT,
                    header::HeaderValue::from_static(USER_AGENT),
                );
                headers
            })
            .build()
            .unwrap()
    };
}

const REDIRECT_URI: &str = dotenv!("REDDIT_REDIRECT_URI");
const USER_AGENT: &str = dotenv!("REDDIT_USER_AGENT");
const CLIENT_ID: &str = dotenv!("REDDIT_CLIENT_ID");
const SECRET: &str = dotenv!("REDDIT_SECRET");
const PERMANENT: bool = true;
const SCOPES: &[Scope] = {
    use Scope::*;
    &[
        Account,  // Find language
        Identity, // Find username
        Submit,   // Submit threads
        Edit,     // Update threads
        ModPosts, // (Moderators) Approve a post so it's visible
        ModFlair, // (Moderators) Add/remove/edit a flair on the submission
    ]
};

#[inline(always)]
fn endpoint(path: &str) -> String {
    format!("https://oauth.reddit.com{}", path)
}

pub struct RedditUser {
    pub refresh_token: String,
    pub access_token:  Option<String>,
    pub expires_at:    Instant,
}

impl RedditUser {
    #[inline]
    pub fn get_auth_url(callback: &str) -> Result<String, Box<dyn Error>> {
        Ok(Url::parse_with_params(
            "https://ssl.reddit.com/api/v1/authorize",
            &[
                ("response_type", "code"),
                ("client_id", CLIENT_ID),
                ("state", callback),
                ("redirect_uri", REDIRECT_URI),
                (
                    "duration",
                    match PERMANENT {
                        true => "permanent",
                        false => "temporary",
                    },
                ),
                ("scope", &SCOPES.iter().join(" ")),
            ],
        )?
        .into_string())
    }

    #[inline]
    pub fn obtain_refresh_token(code: &str) -> Result<RedditUser, request::Error> {
        #[derive(Deserialize)]
        struct APIReturnType {
            access_token:  Option<String>,
            expires_in:    u64,
            refresh_token: Option<String>,
        }

        if cfg!(test) {
            return Ok(RedditUser {
                refresh_token: guid(),
                access_token:  Some(guid()),
                expires_at:    Instant::now() + Duration::from_secs(3600),
            });
        }

        let data: APIReturnType = CLIENT
            .post("https://ssl.reddit.com/api/v1/access_token")
            .basic_auth(CLIENT_ID, Some(SECRET))
            .form(&[
                ("grant_type", "authorization_code"),
                ("code", code),
                ("redirect_uri", REDIRECT_URI),
            ])
            .send()?
            .json()?;

        Ok(RedditUser {
            refresh_token: data.refresh_token.unwrap(),
            access_token:  data.access_token,
            expires_at:    Instant::now() + Duration::from_secs(data.expires_in),
        })
    }

    #[inline]
    fn me(&self) -> ResponseResult {
        CLIENT
            .get(&endpoint("/api/v1/me"))
            .bearer_auth(&self.access_token.clone().unwrap())
            .send()
    }

    #[inline]
    pub fn username(&self) -> Result<String, request::Error> {
        if cfg!(test) {
            return Ok(guid());
        }

        // We may use the `is_mod` field in the future
        // to automatically determine if the user is a moderator
        // of a specific subreddit.
        #[derive(Deserialize)]
        struct APIReturnType {
            name: String,
            #[allow(unused)]
            is_mod: bool,
        }

        Ok(self.me()?.json::<APIReturnType>()?.name)
    }

    #[inline]
    fn prefs(&self) -> ResponseResult {
        CLIENT
            .get(&endpoint("/api/v1/me/prefs"))
            .bearer_auth(&self.access_token.clone().unwrap())
            .send()
    }

    #[inline]
    pub fn lang(&self) -> Result<String, request::Error> {
        if cfg!(test) {
            return Ok(guid()[0..2].to_owned());
        }

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
