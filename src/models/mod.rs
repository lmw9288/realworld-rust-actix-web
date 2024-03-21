use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: u64,
}

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
    // pub password: String,

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

#[derive(Debug, Deserialize, Serialize)]
pub struct ArticlesWrapper<T>
where
    T: serde::Serialize,
{
    pub articles: Vec<T>,
    #[serde(rename = "articlesCount")]
    pub articles_count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleWrapper<T>
where
    T: serde::Serialize,
{
    pub article: T,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleCreateForm {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tagList: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleUpdateForm {
    pub title: String,
    pub description: String,
    pub body: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleResponse {
    pub title: String,
    pub slug: String,
    pub body: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub description: String,
    pub favorited: bool,
    #[serde(rename = "favoritesCount")]
    pub favorites_count: u32,
    #[serde(rename = "tagList")]
    pub tag_list: Vec<String>,
    pub author: UserResponse,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct ArticleEntity {
    pub id: u64,
    pub title: String,
    pub slug: String,
    pub body: String,
    pub description: String,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: NaiveDateTime,
}
