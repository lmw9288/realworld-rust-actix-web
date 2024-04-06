use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use self::user::UserResponse;

pub mod comment;
pub mod user;
pub mod article;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: u64,
}




#[derive(Debug, Deserialize, Serialize)]
pub struct ProfileWrapper<T>
where
    T: serde::Serialize,
{
    pub profile: T,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProfileResponse {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TagsWrapper {
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct TagEntity {
    pub id: i64,
    pub name: String,
    pub article_id: i64,
    pub user_id: i64,
}

