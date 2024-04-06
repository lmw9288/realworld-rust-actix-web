use sqlx::MySqlPool;

use crate::models::comment::CommentEntity;

use super::PersistenceError;

pub async fn select_comments_by_article_id(
    pool: &MySqlPool,
    article_id: i64,
) -> Result<Vec<CommentEntity>, PersistenceError> {
    let comments = sqlx::query_as!(
        CommentEntity,
        "select * from comment where article_id = ? order by id desc",
        article_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(comments)
}

pub async fn get_comment_by_id(
    pool: &MySqlPool,
    comment_id: i64,
) -> Result<CommentEntity, PersistenceError> {
    let comment = sqlx::query_as!(
        CommentEntity,
        "select * from comment where id = ? limit 1",
        comment_id,
    )
    .fetch_one(pool)
    .await?;
    Ok(comment)
}

pub async fn insert_article_comment(
    pool: &MySqlPool,
    user_id: i64,
    body: String,
    article_id: i64,
) -> Result<i64, PersistenceError> {
    let result = sqlx::query!(
        "INSERT INTO comment (created_at, updated_at, body, user_id, article_id) VALUES (?, ?, ?, ?, ?)",
        chrono::Utc::now().naive_utc(),
        chrono::Utc::now().naive_utc(),
        body,
        user_id,
        article_id
    )
    .execute(pool)
    .await?;
    if result.last_insert_id() > 0 {
        Ok(result.last_insert_id() as i64)
    } else {
        Err(PersistenceError::Unknown)
    }
}

pub async fn delete_comment_by_id(
    pool: &MySqlPool,
    comment_id: i64,
) -> Result<(), PersistenceError> {
    let reuslt = sqlx::query!("DELETE FROM comment WHERE id = ?", comment_id)
        .execute(pool)
        .await
        .unwrap();
    if reuslt.rows_affected() > 0 {
        Ok(())
    } else {
        Err(PersistenceError::Unknown)
    }
}
