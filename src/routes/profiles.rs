use actix_web::{post, Responder, web};
use sqlx::MySqlPool;

use realworld_rust_actix_web::SessionState;

use crate::models::{ProfileResponse, ProfileWrapper};
use crate::persistence::{follow_user_by, select_user_by_username};

#[post("/{username}/follow")]
pub async fn follow_user(
    session_state: SessionState,
    path: web::Path<(String)>,
    pool: web::Data<MySqlPool>,
) -> actix_web::Result<impl Responder> {
    let user_id = session_state.user_id;

    let (username) = path.into_inner();

    let user = select_user_by_username(&pool, username).await?;
    let _last_insert_id = follow_user_by(&pool, user_id, user.id).await?;

    Ok(web::Json(ProfileWrapper {
        profile: ProfileResponse {
            username: user.username,
            bio: None,
            image: None,
            following: true,
        }
    }))
}
