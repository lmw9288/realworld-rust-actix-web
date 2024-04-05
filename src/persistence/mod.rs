use actix_web::http::StatusCode;
use derive_more::{Display, Error, From};
use sqlx::MySqlPool;


pub mod article;
pub mod user;
pub mod tag;

#[derive(Debug, Display, Error, From)]
pub enum PersistenceError {
    // EmptyBankName,
    // EmptyCountry,
    // EmptyBranch,
    // EmptyLocation,
    // EmptyTellerName,
    // EmptyCustomerName,
    MysqlError(sqlx::Error),

    Unknown,
}

impl actix_web::ResponseError for PersistenceError {
    fn status_code(&self) -> StatusCode {
        match self {
            // PersistenceError::EmptyBankName
            // | PersistenceError::EmptyCountry
            // | PersistenceError::EmptyBranch
            // | PersistenceError::EmptyLocation
            // | PersistenceError::EmptyTellerName
            // | PersistenceError::EmptyCustomerName => StatusCode::BAD_REQUEST,
            PersistenceError::MysqlError(e) => {
                log::error!("PersistenceError {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            PersistenceError::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}



pub async fn insert_follow_by_user(
    pool: &MySqlPool,
    user_id: i64,
    followee_user_id: i64,
) -> Result<i64, PersistenceError> {
    let result = sqlx::query!(
        "insert user_follow(follower_user_id, followee_user_id) values (?, ?)",
        user_id,
        followee_user_id
    )
    .execute(pool)
    .await?;

    if result.last_insert_id() > 0 {
        Ok(result.last_insert_id() as i64)
    } else {
        Err(PersistenceError::Unknown)
    }
}


pub async fn delete_follow_by_user(
    pool: &MySqlPool,
    user_id: i64,
    followee_user_id: i64,
) -> Result<(), PersistenceError> {
    let result = sqlx::query!(
        "delete from user_follow where follower_user_id = ? and followee_user_id = ?",
        user_id,
        followee_user_id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() > 0 {
        Ok(())
    } else {
        Err(PersistenceError::Unknown)
    }
}
