use chrono::Utc;
use jsonwebtoken as jwt;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref HEADER: jwt::Header = jwt::Header::default();
    static ref VALIDATION: jwt::Validation = jwt::Validation {
        validate_iat: true,
        validate_exp: false,
        ..jwt::Validation::default()
    };
}

const ROCKET_SECRET_KEY: &[u8] = dotenv!("ROCKET_SECRET_KEY").as_bytes();

#[derive(Serialize, Deserialize)]
pub struct Claim {
    user_id: i32,
    iat: i64,
}

impl Claim {
    #[inline]
    pub fn new(user_id: i32) -> Claim {
        Claim {
            user_id,
            iat: Utc::now().timestamp(),
        }
    }

    #[inline]
    pub fn encode(&self) -> Result<String, jsonwebtoken::errors::Error> {
        jwt::encode(&HEADER, self, ROCKET_SECRET_KEY)
    }

    #[inline]
    pub fn get_user_id(token: &str) -> Result<i32, jsonwebtoken::errors::Error> {
        Ok(jwt::decode::<Claim>(token, ROCKET_SECRET_KEY, &VALIDATION)?
            .claims
            .user_id)
    }
}
