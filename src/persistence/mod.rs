use actix_web::http::StatusCode;
use chrono::{Local, Utc};
use derive_more::{Display, Error, From};
use slugify::slugify;
use sqlx::any::AnyValue;
use sqlx::{Encode, Execute, MySqlPool, QueryBuilder};

use crate::models::{ArticleCreateForm, ArticleEntity, UserEntity, UserUpdateForm};
use crate::utils::encrypt_password;

#[derive(Debug, Display, Error, From)]
pub enum PersistenceError {
    EmptyBankName,
    EmptyCountry,
    EmptyBranch,
    EmptyLocation,
    EmptyTellerName,
    EmptyCustomerName,

    MysqlError(sqlx::Error),

    Unknown,
}

impl actix_web::ResponseError for PersistenceError {
    fn status_code(&self) -> StatusCode {
        match self {
            PersistenceError::EmptyBankName
            | PersistenceError::EmptyCountry
            | PersistenceError::EmptyBranch
            | PersistenceError::EmptyLocation
            | PersistenceError::EmptyTellerName
            | PersistenceError::EmptyCustomerName => StatusCode::BAD_REQUEST,

            PersistenceError::MysqlError(_) | PersistenceError::Unknown => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

pub async fn insert_user(
    pool: &MySqlPool,
    username: String,
    email: String,
    password: String,
) -> Result<u64, PersistenceError> {
    let hash_password = encrypt_password(password);

    let result = sqlx::query!(
        "INSERT INTO user (username, email, password) VALUES (?, ?, ?)",
        username,
        email,
        hash_password,
    )
    .execute(pool)
    .await?;

    if result.last_insert_id() > 0 {
        Ok(result.last_insert_id())
    } else {
        Err(PersistenceError::Unknown)
    }
}

pub async fn select_user_by_id(pool: &MySqlPool, id: i64) -> Result<UserEntity, PersistenceError> {
    let user = sqlx::query_as!(
        UserEntity,
        "SELECT id, username, email, password FROM user WHERE id = ? limit 1",
        (id)
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}
//
pub async fn select_user_by_email(
    pool: &MySqlPool,
    email: String,
) -> Result<UserEntity, PersistenceError> {
    let user = sqlx::query_as!(
        UserEntity,
        "SELECT id, username, email, password FROM user WHERE email = ? limit 1",
        (email)
    )
    .fetch_one(pool)
    .await?;
    Ok(user)
}
//
pub async fn update_user_by_id(
    pool: &MySqlPool,
    id: i64,
    update_form: UserUpdateForm,
) -> Result<(), PersistenceError> {
    // let mut conn = pool.get_conn()?;

    // 设置要更新的字段和对应的值
    let mut fields_values = vec![];
    if update_form.username.is_some() {
        fields_values.push(("username", update_form.username.unwrap()))
    }
    if update_form.email.is_some() {
        fields_values.push(("email", update_form.email.unwrap()))
    }
    if update_form.password.is_some() {
        fields_values.push(("password", encrypt_password(update_form.password.unwrap())));
    }
    if update_form.bio.is_some() {
        fields_values.push(("bio", update_form.bio.unwrap()))
    }
    if update_form.image.is_some() {
        fields_values.push(("image", update_form.image.unwrap()))
    }

    // 构建 SQL 更新语句
    let mut query_builder = QueryBuilder::new("UPDATE user SET ");
    for (i, (column, value)) in fields_values.iter().enumerate() {
        if i > 0 {
            query_builder.push(", ");
        }
        query_builder.push(column);
        query_builder.push(" = ");
        query_builder.push_bind(value);
    }

    query_builder.push(" WHERE id = ");
    query_builder.push_bind(id);

    let query = query_builder.build();
    log::info!("update user sql: {:?}", query.sql());
    let t = query.execute(pool).await?;

    if t.rows_affected() > 0 {
        log::info!("update user success");
        Ok(())
    } else {
        log::info!("update user error");
        Err(PersistenceError::Unknown)
    }
}
//
pub async fn insert_article(
    pool: &MySqlPool,
    create_form: ArticleCreateForm,
    user_id: i64,
) -> Result<u64, PersistenceError> {
    // let mut conn = pool.get_conn()?;

    let result = sqlx::query!(
        "insert into article(title, slug, description, body, created_at, updated_at, user_id) values (?, ?, ?, ?, ?, ?, ?)",
        create_form.title,
        slugify::slugify!(&create_form.title),
        create_form.description,
        create_form.body,
        Utc::now().naive_utc(),
        Utc::now().naive_utc(),
        user_id
    ).execute(pool).await?;

    if result.last_insert_id() > 0 {
        Ok(result.last_insert_id())
    } else {
        Err(PersistenceError::Unknown)
    }
}
//
pub async fn select_article_by_id(
    pool: &MySqlPool,
    id: u64,
) -> Result<ArticleEntity, PersistenceError> {
    // let mut conn = pool.get_conn()?;

    // 使用参数化查询以避免SQL注入风险
    let article = sqlx::query_as!(ArticleEntity,
            "SELECT id, title, slug, description, body, created_at, updated_at FROM article WHERE id = ? limit 1",
        (id)

        ).fetch_one(pool).await?;
    Ok(article)
}
//
// pub fn update_article_by_slug(pool: &Pool) {
//
// }
