use actix_web::http::StatusCode;
use derive_more::{Display, Error, From};

pub mod article;
pub mod tag;
pub mod user;

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


