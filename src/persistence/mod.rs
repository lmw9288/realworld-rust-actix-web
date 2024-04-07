

use actix_web::{body::BoxBody, http::StatusCode, HttpResponse};
use derive_more::{Display, Error, From};
use realworld_rust_actix_web::ServiceError;

pub mod article;
pub mod tag;
pub mod user;
pub mod comment;

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
    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            // PersistenceError::EmptyBankName
            // | PersistenceError::EmptyCountry
            // | PersistenceError::EmptyBranch
            // | PersistenceError::EmptyLocation
            // | PersistenceError::EmptyTellerName
            // | PersistenceError::EmptyCustomerName => StatusCode::BAD_REQUEST,
            PersistenceError::MysqlError(e) => {
                log::error!("PersistenceError {}", e);

                HttpResponse::with_body(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    serde_json::to_string(&ServiceError::new(e.to_string())).unwrap(),
                )
                .map_into_boxed_body()
            }
            PersistenceError::Unknown => HttpResponse::with_body(
                StatusCode::INTERNAL_SERVER_ERROR,
                serde_json::to_string(&ServiceError::new("unknown error".to_string())).unwrap(),
            )
            .map_into_boxed_body(),
        }
    }
}
