use std::{
    borrow::Cow,
    env, fmt,
    sync::{Arc, Mutex, RwLock},
    time::{SystemTime, UNIX_EPOCH},
};

use actix_web::{
    dev::Service,
    dev::{Payload, ServiceRequest},
    error::ErrorUnauthorized,
    http::header,
    http::header::{Header, HeaderValue, InvalidHeaderValue, TryIntoHeaderValue},
    middleware::Logger,
    web,
    web::{BufMut, BytesMut},
    App, Error, FromRequest, HttpMessage, HttpRequest, HttpResponse, HttpServer, Result,
};
use actix_web_httpauth::{
    extractors::bearer::{BearerAuth, Config},
    extractors::{bearer, AuthExtractorConfig, AuthenticationError},
    headers::authorization,
    headers::authorization::{ParseError, Scheme},
    middleware::HttpAuthentication,
};
use dotenvy::dotenv;
use env_logger::Env;
use futures::{
    future::{err, ok, LocalBoxFuture, Ready},
    FutureExt,
};
use jsonwebtoken::{DecodingKey, Validation};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, MySqlPool, Pool};

use crate::models::Claims;

mod models;
mod persistence;
mod routes;
mod utils;

async fn get_conn_builder() -> MySqlPool {
    let database_url = env::var("DATABASE_URL").expect("database url is empty!!!");
    // We create a single connection pool for SQLx that's shared across the whole application.
    // This saves us from opening a new connection for every API call, which is wasteful.
    MySqlPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("could not connect to database_url")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    log::info!("initializing database connection");

    let i = Box::new(5);

    let pool = get_conn_builder().await;

    let pool_data = web::Data::new(pool);
    HttpServer::new(move || {
        App::new()
            .app_data(pool_data.clone())
            .wrap(Logger::default())
            .service(
                // 不需要登录的服务
                web::scope("/api/users")
                    .service(routes::users::login_user)
                    .service(routes::users::registry_user),
            )
            .service(
                web::scope("/api/articles")
                    .service(routes::articles::list_articles)
                    .service(routes::articles::create_article), //         .service(routes::list_articles_feed)
                                                                //         .service(routes::single_article),
            )
            .service(
                web::scope("/api/user")
                    .service(routes::users::current_user)
                    .service(routes::users::update_user),
            )
            .service(web::scope("/api/profiles").service(routes::profiles::follow_user))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
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
