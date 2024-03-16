use std::ops::Add;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use actix_web::{error, Error, get, HttpMessage, HttpRequest, post, put, Responder, Result, web};
use actix_web::dev::ServiceRequest;
use actix_web::http::StatusCode;
use jsonwebtoken::{EncodingKey, Header};
use mysql::Pool;

use realworld_rust_actix_web::SessionState;

use crate::models::{
    Claims, UserLogin, UserRegistryForm, UserResponse, UserUpdateForm, UserWrapper,
};
use crate::persistence::{insert_user, select_user_by_email, select_user_by_id, update_user_by_id};
use crate::utils::verify_password;

#[post("/login")]
pub async fn login_user(
    json: web::Json<UserWrapper<UserLogin>>,
    pool: web::Data<Pool>,
) -> Result<impl Responder> {
    // println!("login_user: {:?}", json);
    // let email = json.email;
    // let password = json.password;
    let UserLogin { email, password } = json.into_inner().user;

    let user = web::block(move || select_user_by_email(&pool, email)).await??;

    // log::info!("login_user: {:?}", user);

    // 创建 JWT 的 payload
    let my_claims = Claims {
        sub: user.id,
        exp: SystemTime::now()
            .add(Duration::from_secs(60 * 60 * 2))
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    // 生成 JWT
    let token = jsonwebtoken::encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret("realworld".as_ref()),
    )
    .unwrap();
    if verify_password(password, &user.password) {
        Ok(web::Json(UserWrapper {
            user: UserResponse {
                username: user.username,
                email: user.email,
                token,
                bio: None,
                image: None,
            },
        }))
    } else {
        log::error!("invalid email or password");
        Err(error::ErrorUnauthorized("invalid email or password"))
    }
}

#[post("")]
pub async fn registry_user(
    json: web::Json<UserWrapper<UserRegistryForm>>,
    pool: web::Data<Pool>,
) -> Result<impl Responder> {
    let UserRegistryForm {
        username,
        email,
        password,
    } = json.into_inner().user;

    let user = web::block(move || {
        let last_insert_id = insert_user(&pool, username, email, password)?;
        select_user_by_id(&pool, last_insert_id)
    })
    .await??;

    // 创建 JWT 的 payload
    let my_claims = Claims {
        sub: user.id,
        exp: SystemTime::now()
            .add(Duration::from_secs(60 * 60 * 2))
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    // 生成 JWT
    let token = jsonwebtoken::encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret("realworld".as_ref()),
    )
    .unwrap();

    Ok(web::Json(UserWrapper {
        user: UserResponse {
            username: user.username,
            email: user.email,
            token,
            bio: None,
            image: None,
        },
    }))
}

#[get("")]
pub async fn current_user(
    session_state: SessionState,
    pool: web::Data<Pool>,
) -> Result<impl Responder> {
    log::info!("current_user: session_state: {:?}", session_state);
    let SessionState { user_id, token } = session_state;

    let user = web::block(move || {
        let user = select_user_by_id(&pool, user_id);
        user
    })
    .await??;
    Ok(web::Json(UserWrapper {
        user: UserResponse {
            username: user.username,
            email: user.email,
            token,
            bio: None,
            image: None,
        },
    }))
}

#[put("")]
pub async fn update_user(
    session_state: SessionState,
    pool: web::Data<Pool>,
    json: web::Json<UserWrapper<UserUpdateForm>>,
) -> Result<impl Responder> {
    let SessionState { user_id, token } = session_state;

    let user = web::block(move || {
        update_user_by_id(&pool, user_id, json.into_inner().user)?;
        select_user_by_id(&pool, user_id)
    })
    .await??;

    Ok(web::Json(UserWrapper {
        user: UserResponse {
            username: user.username,
            email: user.email,
            token,
            bio: None,
            image: None,
        },
    }))
}
