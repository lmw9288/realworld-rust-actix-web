use std::fmt;

use crate::models::Claims;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest, HttpResponse, ResponseError};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

mod models;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthTokenError {
    pub errors: ErrorsBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorsBody {
    body: Vec<String>,
}

impl AuthTokenError {
    pub fn new(msg: String) -> Self {
        AuthTokenError {
            errors: ErrorsBody { body: vec![msg] },
        }
    }
}

impl ResponseError for AuthTokenError {
    // fn status_code(&self) -> actix_web::http::StatusCode {
    //     actix_web::http::StatusCode::UNAUTHORIZED
    // }
    fn error_response(&self) -> actix_web::HttpResponse {
        HttpResponse::Unauthorized().json(self)
    }
}

impl fmt::Display for AuthTokenError {
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
    type Error = AuthTokenError;
    type Future = Ready<actix_web::Result<SessionState, AuthTokenError>>;
    // type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let auth = req.headers().get("Authorization");
        // log::info!("Authorization: {:?}", auth);

        match auth {
            Some(auth) => {
                let _split: Vec<&str> = auth.to_str().unwrap().split("Token").collect();
                let token = _split[1].trim();

                let token_data = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret("realworld".as_ref()),
                    &Validation::default(),
                );
                match token_data {
                    Ok(token_data) => {
                        let user_id = token_data.claims.sub;
                        ok(SessionState {
                            user_id,
                            token: token.to_string(),
                        })
                    }
                    Err(_e) => err(AuthTokenError::new("invalid token!".to_string())),
                }
            }
            None => err(AuthTokenError::new("invalid header!".to_string())),
        }
    }
}
