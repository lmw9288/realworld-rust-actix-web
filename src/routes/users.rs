use std::ops::Add;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use actix_web::{error, get, post, put, Responder, web};
use jsonwebtoken::{EncodingKey, Header};
use sqlx::MySqlPool;
use realworld_rust_actix_web::SessionState;
use crate::models::{Claims, UserLogin, UserRegistryForm, UserResponse, UserUpdateForm, UserWrapper};
use crate::persistence::user::{insert_user, select_user_by_email, select_user_by_id, update_user_by_id};
use crate::utils::verify_password;

#[post("")]
pub async fn registry_user(
    json: web::Json<UserWrapper<UserRegistryForm>>,
    pool: web::Data<MySqlPool>,
) -> actix_web::Result<impl Responder> {
    let UserRegistryForm {
        username,
        email,
        password,
    } = json.into_inner().user;

    let last_insert_id = insert_user(&pool, username, email, password).await?;
    let user = select_user_by_id(&pool, last_insert_id as i64).await?;

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
            token: Some(token),
            bio: None,
            image: None,
        },
    }))
}

#[post("/login")]
pub async fn login_user(
    json: web::Json<UserWrapper<UserLogin>>,
    pool: web::Data<MySqlPool>,
) -> actix_web::Result<impl Responder> {
    // println!("login_user: {:?}", json);
    // let email = json.email;
    // let password = json.password;
    let UserLogin { email, password } = json.into_inner().user;

    let user = select_user_by_email(&pool, email).await?;

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
                token: Some(token),
                bio: None,
                image: None,
            },
        }))
    } else {
        log::error!("invalid email or password");
        Err(error::ErrorUnauthorized("invalid email or password"))
    }
}
//
#[get("")]
pub async fn current_user(
    session_state: SessionState,
    pool: web::Data<MySqlPool>,
) -> actix_web::Result<impl Responder> {
    log::info!("current_user: session_state: {:?}", session_state);
    let SessionState { user_id, token } = session_state;

    let user = select_user_by_id(&pool, user_id).await?;
    Ok(web::Json(UserWrapper {
        user: UserResponse {
            username: user.username,
            email: user.email,
            token: Some(token),
            bio: None,
            image: None,
        },
    }))
}
//
#[put("")]
pub async fn update_user(
    session_state: SessionState,
    pool: web::Data<MySqlPool>,
    json: web::Json<UserWrapper<UserUpdateForm>>,
) -> actix_web::Result<impl Responder> {
    let SessionState { user_id, token } = session_state;

    update_user_by_id(&pool, user_id, json.into_inner().user).await?;
    let user = select_user_by_id(&pool, user_id).await?;

    Ok(web::Json(UserWrapper {
        user: UserResponse {
            username: user.username,
            email: user.email,
            token: Some(token),
            bio: None,
            image: None,
        },
    }))
}