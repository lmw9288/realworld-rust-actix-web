use mysql::prelude::FromRow;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub(crate) sub: u64,
    pub(crate) exp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserWrapper<T>
    where
        T: serde::Serialize,
{
    pub user: T,
}


#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct UserEntity {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserRegistryForm {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub username: String,
    pub email: String,
    pub token: String,
    // pub password: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}
