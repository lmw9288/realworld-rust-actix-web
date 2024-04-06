use serde::{Deserialize, Serialize};

use super::UserResponse;

#[derive(Debug, Deserialize, Serialize)]
pub struct CommentEntity {
    pub id: i64,
    pub body: String,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::NaiveDateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::NaiveDateTime,
    pub article_id: i64,
    pub user_id: i64,
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
pub struct CommentResponse {
    pub id: i64,
    pub body: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub author: UserResponse,
}