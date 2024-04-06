use std::fmt;

use crate::models::Claims;
use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

mod models;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub errors: ErrorsBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorsBody {
    body: Vec<String>,
}

impl ErrorResponse {
    // pub fn new(errors: Vec<String>) -> Self {
    //     ErrorResponse {
    //         errors: ErrorsBody { body: errors },
    //     }
    // }

    pub fn new(msg: String) -> Self {
        ErrorResponse {
            errors: ErrorsBody { body: vec![msg] },
        }
    }
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.errors)
    }
}

#[derive(Debug, Clone)]
pub struct SessionState {
    pub user_id: i64,
    pub token: String,
}

impl FromRequest for SessionState {
    type Error = Error;
    type Future = Ready<actix_web::Result<SessionState, Error>>;
    // type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let auth = req.headers().get("Authorization");
        // log::info!("Authorization: {:?}", auth);

        match auth {
            Some(auth) => {
                let _split: Vec<&str> = auth.to_str().unwrap().split("Token").collect();
                let token = _split[1].trim();

                // log::info!("token: {}", token);

                // let _config: Config = Config {};
                // let _var = _config.get_config_with_key("SECRET_KEY");
                // let key = _var.as_bytes();
                match decode::<Claims>(
                    token,
                    &DecodingKey::from_secret("realworld".as_ref()),
                    &Validation::default(),
                ) {
                    Ok(token_data) => {
                        let user_id = token_data.claims.sub;
                        ok(SessionState {
                            user_id,
                            token: token.to_string(),
                        })
                    }
                    Err(_e) => {
                        let error_response = ErrorResponse::new("invalid token!".to_string());
                        err(ErrorUnauthorized(
                            serde_json::to_string(&error_response).unwrap(),
                        ))
                    }
                }
            }
            None => err(ErrorUnauthorized("blocked!")),
        }
    }
}
