use chrono::Utc;
use jsonwebtoken as jwt;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct Claim {
    user_id: i32,
    iat: i64,
}

impl Claim {
    pub fn new(user_id: i32) -> Claim {
        Claim {
            user_id,
            iat: Utc::now().timestamp(),
        }
    }

    pub fn encode(&self) -> Result<String, jsonwebtoken::errors::Error> {
        jwt::encode(
            &jwt::Header::default(),
            self,
            env::var("ROCKET_SECRET_KEY").unwrap().as_bytes(),
        )
    }

    pub fn get_user_id(token: &str) -> Result<i32, jsonwebtoken::errors::Error> {
        let validation = jwt::Validation {
            validate_iat: true,
            validate_exp: false,
            ..jwt::Validation::default()
        };

        jwt::decode::<Claim>(
            token,
            env::var("ROCKET_SECRET_KEY").unwrap().as_bytes(),
            &validation,
        )
        .map(|data| data.claims.user_id)
    }
}
