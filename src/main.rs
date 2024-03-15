use std::{
    env,
    fmt,
    borrow::Cow,
    sync::{Arc, Mutex, RwLock},
    time::{SystemTime, UNIX_EPOCH},
};

use actix_web::{
    App,
    dev::{Payload, ServiceRequest},
    dev::Service,
    Error,
    error::ErrorUnauthorized,
    FromRequest,
    http::header,
    http::header::{Header, HeaderValue, InvalidHeaderValue, TryIntoHeaderValue},
    HttpMessage,
    HttpRequest,
    HttpResponse,
    HttpServer,
    middleware::Logger,
    Result,
    web,
    web::{BufMut, BytesMut},
};
use actix_web_httpauth::{
    extractors::{AuthenticationError, AuthExtractorConfig, bearer},
    extractors::bearer::{BearerAuth, Config},
    headers::authorization,
    headers::authorization::{ParseError, Scheme},
    middleware::HttpAuthentication,
};
use dotenvy::dotenv;
use env_logger::Env;
use futures::{
    future::{err, LocalBoxFuture, ok, Ready},
    FutureExt,
};
use jsonwebtoken::{DecodingKey, Validation};

use crate::models::Claims;

mod models;
mod routes;
mod persistence;
mod utils;

fn get_conn_builder() -> mysql::OptsBuilder {
    mysql::OptsBuilder::new()
        .ip_or_hostname(Some("127.0.0.1"))
        .tcp_port(3306)
        .db_name(Some("realworld"))
        .user(Some("root"))
        .pass(Some(""))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    log::info!("initializing database connection");


    let builder = get_conn_builder();

    let pool = mysql::Pool::new(builder).unwrap();


    let pool_data = web::Data::new(pool);
    HttpServer::new(move || {
        App::new()
            .app_data(pool_data.clone())
            .wrap(Logger::default())
            .service(
                // 不需要登录的服务
                web::scope("/api/users")
                    .service(routes::login_user)
                    .service(routes::registry_user)
            )
            .service(
                web::scope("/api/user")
                    .service(routes::current_user)
            )
    })
        .bind(("127.0.0.1", 3001))?
        .run()
        .await
}

#[derive(Debug, Clone)]
pub struct SessionState {
    user_id: u64,
    token: String,
}

impl FromRequest for SessionState {
    type Error = Error;
    type Future = Ready<Result<SessionState, Error>>;
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
                match jsonwebtoken::decode::<Claims>(
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

// async fn authenticate(req: ServiceRequest, auth: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
//     // 从 HTTP 请求中提取 JWT
//     let token = auth.token();
//
//     log::info!("token: {}", token);
//
//     // 配置 JSON Web Token 解码密钥和验证选项
//     let key = DecodingKey::from_secret("realworld".as_ref());
//     let validation = Validation::default();
//
//     // 尝试解码 JWT
//     match jsonwebtoken::decode::<Claims>(&token, &key, &validation) {
//         Ok(token_data) => {
//             // 解码成功，进行身份验证
//             let user_id = token_data.claims.sub;
//             // 这里可以根据用户 ID 或其他声明来进行具体的身份验证逻辑
//             // let mut request = req.clone();
//             req.extensions_mut().insert(SessionState {
//                 user_id
//             });
//             // 身份验证成功，返回 HTTP 响应
//             Ok(req)
//         }
//         Err(_) => {
//             // 解码失败或验证失败，返回未经授权的 HTTP 响应
//             let config = req.app_data::<bearer::Config>()
//                 .cloned()
//                 .unwrap_or_default();
//             // .scope("urn:example:channel=HBO&urn:example:rating=G,PG-13");
//
//             Err((AuthenticationError::from(config).into(), req))
//         }
//     }
// }