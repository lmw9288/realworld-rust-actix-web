use actix_web::{delete, get, post, web, Responder};
use realworld_rust_actix_web::SessionState;
use sqlx::MySqlPool;

use crate::models::{to_profile_response, ProfileResponse, ProfileWrapper};
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
        profile: to_profile_response(target_user, following),
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

    let target_user = select_user_by_username(&pool, username).await?;
    let _last_insert_id = insert_follow_by_user(&pool, user_id, target_user.id).await?;

    Ok(web::Json(ProfileWrapper {
        profile: to_profile_response(target_user, true),
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

    let target_user = select_user_by_username(&pool, username).await?;

    delete_follow_by_user(&pool, user_id, target_user.id).await?;

    Ok(web::Json(ProfileWrapper {
        profile: to_profile_response(target_user, false),
    }))
}
