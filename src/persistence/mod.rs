use actix_web::http::StatusCode;
use derive_more::{Display, Error, From};
use mysql::{Error, params, Pool};
use mysql::prelude::*;
use crate::models::UserEntity;
use crate::utils::encrypt_password;

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

    let hash_password = encrypt_password(password);
    
    let last_insert_id = conn.exec_drop(
        "
        INSERT INTO user (username, email, password)
            VALUES (:username, :email, :password)
        ",
        params! {
            "username" => username,
            "email" => email,
            "password" => hash_password,

        },
    )
        .map(|_| conn.last_insert_id())?;

    if last_insert_id > 0 {
        Ok(last_insert_id)
    } else {
        Err(PersistenceError::Unknown)
    }
}

pub fn select_user_by_id(pool: &Pool, id: u64) -> Result<UserEntity, PersistenceError> {
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

pub fn select_user_by_email(pool: &Pool, email: String) -> Result<UserEntity, PersistenceError> {
    let mut conn = pool.get_conn()?;

    // 使用参数化查询以避免SQL注入风险
    let user = conn.exec_map(
        "SELECT id, username, email, password FROM user WHERE email = :email limit 1",
        params! {"email" => email}, |(id, username, email, password)| {
            UserEntity {
                id,
                username,
                email,
                password,
            }
        },
    )?.into_iter().next();
    match user {
        None => Err(PersistenceError::Unknown),
        Some(user) => Ok(user),
    }
}