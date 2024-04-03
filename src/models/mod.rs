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
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleQuery {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,

    pub limit: Option<i32>,
    pub offset: Option<i32>,
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
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub body: String,
    pub description: String,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::NaiveDateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::NaiveDateTime,
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
pub struct CommentResponse {
    pub id: i64,
    pub body: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub author: UserResponse,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommentCreateForm {
    pub body: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommentWrapper<T>
where
    T: serde::Serialize,
{
    pub comment: T,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommentsWrapper<T> {
    pub comments: Vec<T>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TagsWrapper {
    pub tags: Vec<String>,
}
