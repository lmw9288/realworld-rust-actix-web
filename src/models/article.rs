use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use super::user::UserResponse;

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArticleCreateForm {
    pub title: String,
    pub description: String,
    pub body: String,
    #[serde(rename = "tagList")]
    pub tag_list: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleUpdateForm {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
    #[serde(rename = "tagList")]
    pub tag_list: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleQuery {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,

    pub limit: Option<i32>,
    pub offset: Option<i32>,

    pub feed_user_id: Option<i64>,
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
    pub favorites_count: i64,
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
    pub tag_list: String,
    pub user_id: i64,

    pub favorites_count: i64,
    // pub favorited: bool,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct ArticleFavoriteEntity {
    pub user_id: i64,
    pub article_id: i64,
}
