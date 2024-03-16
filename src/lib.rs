use actix_web::{Error, FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::models::Claims;

mod models;

#[derive(Debug, Clone)]
pub struct SessionState {
    pub user_id: u64,
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
                        ok(SessionState { user_id, token: token.to_string() })
                    }
                    Err(_e) => err(ErrorUnauthorized("invalid token!")),
                }
            }
            None => err(ErrorUnauthorized("blocked!")),
        }
    }
}
