use actix_web::http::StatusCode;
use derive_more::{Display, Error, From};
use mysql::{Error, params, Pool};
use mysql::prelude::*;
use crate::models::UserEntity;

#[derive(Debug, Display, Error, From)]
pub enum PersistenceError {
    EmptyBankName,
    EmptyCountry,
    EmptyBranch,
    EmptyLocation,
    EmptyTellerName,
    EmptyCustomerName,

    MysqlError(mysql::Error),

    Unknown,
}

impl actix_web::ResponseError for PersistenceError {
    fn status_code(&self) -> StatusCode {
        match self {
            PersistenceError::EmptyBankName
            | PersistenceError::EmptyCountry
            | PersistenceError::EmptyBranch
            | PersistenceError::EmptyLocation
            | PersistenceError::EmptyTellerName
            | PersistenceError::EmptyCustomerName => StatusCode::BAD_REQUEST,

            PersistenceError::MysqlError(_) | PersistenceError::Unknown => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

pub fn insert_user(pool: &Pool, username: String, email: String, password: String)
                   -> Result<u64, PersistenceError> {
    let mut conn = pool.get_conn()?;

    let last_insert_id = conn.exec_drop(
        "
        INSERT INTO user (username, email, password)
            VALUES (:username, :email, :password)
        ",
        params! {
            "username" => username,
            "email" => email,
            "password" => password,

        },
    )
        .map(|_| conn.last_insert_id())?;

    if last_insert_id > 0 {
        Ok(last_insert_id)
    } else {
        Err(PersistenceError::Unknown)
    }
}

pub fn select_user(pool: &Pool, id: u64) -> Result<UserEntity, PersistenceError> {
    let mut conn = pool.get_conn()?;

    // 使用参数化查询以避免SQL注入风险
    let user = conn.exec_map(
        "SELECT id, username, email FROM user WHERE id = :id limit 1",
        params! {"id" => id}, |(id, username, email)| {
            UserEntity {
                id,
                username,
                email,
                password: "".to_string(),
            }
        },
    )?.into_iter().next();
    match user {
        None => Err(PersistenceError::Unknown),
        Some(user) => Ok(user),
    }
}