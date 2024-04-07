use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserWrapper<T>
where
    T: serde::Serialize,
{
    pub user: T,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserEntity {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub image: Option<String>,
    pub bio: Option<String>,
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
    pub token: Option<String>,
    // pub password: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct UserFollowEntity {
    pub follower_user_id: i64,
    pub followee_user_id: i64,
}

pub fn to_author(user: UserEntity) -> UserResponse {
    UserResponse {
        username: user.username,
        email: user.email,
        token: None,
        bio: user.bio,
        image: user.image,
    }
}
