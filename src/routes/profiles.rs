use actix_web::{delete, get, post, web, Responder};
use sqlx::MySqlPool;

use realworld_rust_actix_web::SessionState;

use crate::models::{ProfileResponse, ProfileWrapper};
use crate::persistence::user::{delete_follow_by_user, insert_follow_by_user};
use crate::persistence::user::{select_follow_by_user, select_user_by_username};

#[get("/{username}")]
pub async fn get_profile(
    session_state: SessionState,
    path: web::Path<String>,
    pool: web::Data<MySqlPool>,
) -> actix_web::Result<impl Responder> {
    let user_id = session_state.user_id;

    let username = path.into_inner();

    let target_user = select_user_by_username(&pool, username).await?;
    let following = select_follow_by_user(&pool, user_id, target_user.id).await?;

    Ok(web::Json(ProfileWrapper {
        profile: ProfileResponse {
            username: target_user.username,
            bio: None,
            image: None,
            following,
        },
    }))
}

#[post("/{username}/follow")]
pub async fn follow_user(
    session_state: SessionState,
    path: web::Path<String>,
    pool: web::Data<MySqlPool>,
) -> actix_web::Result<impl Responder> {
    let user_id = session_state.user_id;

    let username = path.into_inner();

    let user = select_user_by_username(&pool, username).await?;
    let _last_insert_id = insert_follow_by_user(&pool, user_id, user.id).await?;

    Ok(web::Json(ProfileWrapper {
        profile: ProfileResponse {
            username: user.username,
            bio: None,
            image: None,
            following: true,
        },
    }))
}

#[delete("/{username}/follow")]
pub async fn delete_follow_user(
    session_state: SessionState,
    path: web::Path<String>,
    pool: web::Data<MySqlPool>,
) -> actix_web::Result<impl Responder> {
    let user_id = session_state.user_id;

    let username = path.into_inner();

    let user = select_user_by_username(&pool, username).await?;

    delete_follow_by_user(&pool, user_id, user.id).await?;

    Ok(web::Json(ProfileWrapper {
        profile: ProfileResponse {
            username: user.username,
            bio: None,
            image: None,
            following: false,
        },
    }))
}
