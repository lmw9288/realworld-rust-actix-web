use sqlx::MySqlPool;

use super::PersistenceError;

pub async fn insert_tag(
    pool: &MySqlPool,
    name: String,
    article_id: i64,
    user_id: i64,
) -> Result<i64, PersistenceError> {
    let result = sqlx::query!(
        "INSERT INTO tag (name, article_id, user_id) VALUES (?, ?, ?)",
        name,
        article_id,
        user_id
    )
    .execute(pool)
    .await?;
    if result.last_insert_id() > 0 {
        Ok(result.last_insert_id() as i64)
    } else {
        Err(PersistenceError::Unknown)
    }
}

pub async fn delete_tag_by_article_id(
    pool: &MySqlPool,
    article_id: i64,
) -> Result<(), PersistenceError> {
    let result = sqlx::query!("DELETE FROM tag WHERE article_id = ?", article_id)
        .execute(pool)
        .await?;
    if result.rows_affected() > 0 {
        Ok(())
    } else {
        Err(PersistenceError::Unknown)
    }
}
