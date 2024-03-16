use mysql::prelude::FromRow;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: u64,
    pub exp: u64,
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
pub struct UserUpdateForm {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
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
