use std::fmt;

use actix_web::{
    body::BoxBody,
    http::{header, StatusCode},
    web::BytesMut,
    HttpResponse,
};
use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub errors: ErrorsBody,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorsBody {
    body: Vec<String>,
}

impl ErrorResponse {
    // pub fn new(errors: Vec<String>) -> Self {
    //     ErrorResponse {
    //         errors: ErrorsBody { body: errors },
    //     }
    // }

    pub fn new(msg: String) -> Self {
        ErrorResponse {
            errors: ErrorsBody { body: vec![msg] },
        }
    }
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.errors)
    }
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
                    serde_json::to_string(&ErrorResponse::new(e.to_string())).unwrap(),
                )
                .map_into_boxed_body()
            }
            PersistenceError::Unknown => HttpResponse::with_body(
                StatusCode::INTERNAL_SERVER_ERROR,
                serde_json::to_string(&ErrorResponse::new("unknown error".to_string())).unwrap(),
            )
            .map_into_boxed_body(),
        }
    }
}
