use crate::{
    models::{
        comment::{
            CommentCreateForm, CommentEntity, CommentResponse, CommentWrapper, CommentsWrapper,
        },
        user::to_author,
        user::UserEntity,
    },
    persistence::{
        article::select_article_by_slug,
        comment::{
            delete_comment_by_id, get_comment_by_id, insert_article_comment,
            select_comments_by_article_id,
        },
        user::select_user_by_id,
    },
};
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use realworld_rust_actix_web::SessionState;
use sqlx::MySqlPool;

#[get("/{slug}/comments")]
pub async fn get_article_comments(
    // session_state: SessionState,
    pool: web::Data<MySqlPool>,
    path: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let slug = path.into_inner();
    // let user_id = session_state.user_id;
    let article = select_article_by_slug(&pool, slug).await?;
    let comments = select_comments_by_article_id(&pool, article.id).await?;

    let mut result_comments = vec![];
    for comment in comments {
        let user = select_user_by_id(&pool, comment.user_id).await?;
        let comment = to_comment_response(comment, user);
        result_comments.push(comment);
    }
    Ok(web::Json(CommentsWrapper {
        comments: result_comments,
    }))
}

#[post("/{slug}/comments")]
pub async fn create_article_comments(
    session_state: SessionState,
    pool: web::Data<MySqlPool>,
    path: web::Path<String>,
    data: web::Json<CommentWrapper<CommentCreateForm>>,
) -> actix_web::Result<impl Responder> {
    let slug = path.into_inner();
    let user_id = session_state.user_id;
    let comment_form = data.into_inner().comment;
    let article = select_article_by_slug(&pool, slug).await?;

    let comment_id = insert_article_comment(&pool, user_id, comment_form.body, article.id).await?;
    let comment = get_comment_by_id(&pool, comment_id).await?;
    let user = select_user_by_id(&pool, user_id).await?;
    let comment = to_comment_response(comment, user);
    Ok(web::Json(CommentWrapper { comment }))
}

#[delete("/{slug}/comments/{id}")]
pub async fn delete_article_comment(
    // session_state: SessionState,
    pool: web::Data<MySqlPool>,
    path: web::Path<(String, i64)>,
) -> actix_web::Result<impl Responder> {
    // let user_id = session_state.user_id;
    let (_slug, comment_id) = path.into_inner();

    // let article = select_article_by_slug(&pool, slug).await?;
    // let article_id = article.id;

    delete_comment_by_id(&pool, comment_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

fn to_comment_response(comment: CommentEntity, user: UserEntity) -> CommentResponse {
    CommentResponse {
        id: comment.id,
        body: comment.body,
        created_at: comment
            .created_at
            .format("%Y-%m-%dT%H:%M:%S%.3fZ")
            .to_string(),
        updated_at: comment
            .updated_at
            .format("%Y-%m-%dT%H:%M:%S%.3fZ")
            .to_string(),
        author: to_author(user),
    }
}
