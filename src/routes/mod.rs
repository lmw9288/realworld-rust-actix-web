use std::ops::Add;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use actix_web::{error, get, post, put, web, Responder, Result};
use jsonwebtoken::{EncodingKey, Header};
use realworld_rust_actix_web::SessionState;
use sqlx::{MySql, MySqlPool, Pool};

use crate::models::{
    Claims, UserLogin, UserRegistryForm, UserResponse, UserUpdateForm, UserWrapper,
};
use crate::persistence::{insert_user, select_user_by_email, select_user_by_id, update_user_by_id};
use crate::utils::verify_password;

#[post("")]
pub async fn registry_user(
    json: web::Json<UserWrapper<UserRegistryForm>>,
    pool: web::Data<MySqlPool>,
) -> Result<impl Responder> {
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
) -> Result<impl Responder> {
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
) -> Result<impl Responder> {
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
) -> Result<impl Responder> {
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
//
// #[get("")]
// pub async fn list_articles() -> Result<impl Responder> {
//     Ok(web::Json(ArticlesWrapper::<ArticleResponse> {
//         articles: vec![],
//         articles_count: 0,
//     }))
// }
//
// #[post("")]
// pub async fn create_article(
//     session_state: SessionState,
//     pool: web::Data<Pool>,
//     data: web::Json<ArticleWrapper<ArticleCreateForm>>,
// ) -> Result<impl Responder> {
//     let user_id = session_state.user_id;
//
//     let article = web::block(move || {
//         let last_insert_id = insert_article(&pool, data.0.article, user_id)?;
//         select_article_by_id(&pool, last_insert_id)
//     })
//     .await??;
//
//     // let tz_offset = FixedOffset::east(8 * 3600);
//     // let t = tz_offset.from_local_datetime(&article.created_at).unwrap().to_rfc3339();
//     // log::info!("t = {}", t);
//     Ok(web::Json(ArticleWrapper {
//         article: ArticleResponse {
//             title: article.title,
//             slug: article.slug,
//             description: article.description,
//             body: article.body,
//             created_at: Utc::now().to_rfc3339(),
//             updated_at: Utc::now().to_rfc3339(),
//             favorites_count: 0,
//             favorited: false,
//             tag_list: vec![],
//             author: UserResponse {
//                 username: "".to_owned(),
//                 email: "".to_owned(),
//                 token: None,
//                 bio: None,
//                 image: None,
//             },
//         },
//     }))
// }
//
// #[get("/feed")]
// pub async fn list_articles_feed(
//     session_state: SessionState,
//     pool: web::Data<Pool>,
// ) -> Result<impl Responder> {
//     Ok(web::Json(ArticlesWrapper::<ArticleResponse> {
//         articles: vec![],
//         articles_count: 0,
//     }))
// }
//
// #[get("/{slug}")]
// pub async fn single_article(path: web::Path<(String)>) -> Result<impl Responder> {
//     log::info!("single_article: path: {:?}", path);
//
//     Ok(web::Json(ArticleWrapper {
//         article: ArticleResponse {
//             title: "".to_string(),
//             slug: "".to_string(),
//             description: "".to_string(),
//             body: "".to_string(),
//             created_at: Utc::now().to_rfc3339(),
//             updated_at: Utc::now().to_rfc3339(),
//             favorites_count: 1,
//             favorited: false,
//             tag_list: vec![],
//             author: UserResponse {
//                 username: "".to_string(),
//                 email: "".to_string(),
//                 token: None,
//                 bio: None,
//                 image: None,
//             },
//         },
//     }))
// }

// #[put("/{slug}")]
// pub async fn update_article(
//     session_state: SessionState,
//     pool: web::Data<Pool>,
//     path: web::Path<(String)>,
//     data: web::Json<ArticleWrapper<ArticleUpdateForm>>,
// ) -> Result<impl Responder> {
//     let user_id = session_state.user_id;
//     let (slug) = path.0;
//     let update_form = data.article;
//     let article =
//         web::block(move || update_article_by_slug(&pool, user_id, path.0, data.0.article)).await??;
// }
