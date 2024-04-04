use crate::models::{
    CommentResponse, CommentWrapper, CommentsWrapper, UserResponse,
};
use actix_web::{get, post, web, Responder};

#[get("/{slug}/comments")]
pub async fn get_article_comments(
    // session_state: SessionState,
    // pool: web::Data<MySqlPool>,
    // path: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    // let slug = path.into_inner();
    Ok(web::Json(CommentsWrapper::<CommentResponse> {
        comments: vec![],
    }))
}

#[post("/{slug}/comments")]
pub async fn create_article_comments(
    // pool: web::Data<MySqlPool>,
    // path: web::Path<String>,
    // json: web::Json<CommentWrapper<CommentCreateForm>>,
) -> actix_web::Result<impl Responder> {
    // let slug = path.into_inner();
    Ok(web::Json(CommentWrapper::<CommentResponse> {
        comment: CommentResponse {
            id: 0,
            body: "".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            author: UserResponse {
                username: "".to_string(),
                email: "".to_string(),
                token: None,
                bio: None,
                image: None,
            },
        },
    }))
}
