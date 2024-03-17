use crate::models::{ArticleCreateForm, ArticleEntity, UserEntity, UserUpdateForm};
use crate::utils::encrypt_password;
use actix_web::http::StatusCode;
use derive_more::{Display, Error, From};
use log::info;
use mysql::prelude::*;
use mysql::{params, Error, Params, Pool, QueryWithParams, Value};
use std::collections::HashMap;
use slugify::slugify;

#[derive(Debug, Display, Error, From)]
pub enum PersistenceError {
    EmptyBankName,
    EmptyCountry,
    EmptyBranch,
    EmptyLocation,
    EmptyTellerName,
    EmptyCustomerName,

    MysqlError(mysql::Error),

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

pub fn insert_user(
    pool: &Pool,
    username: String,
    email: String,
    password: String,
) -> Result<u64, PersistenceError> {
    let mut conn = pool.get_conn()?;

    let hash_password = encrypt_password(password);

    let last_insert_id = conn
        .exec_drop(
            "
        INSERT INTO user (username, email, password)
            VALUES (:username, :email, :password)
        ",
            params! {
                "username" => username,
                "email" => email,
                "password" => hash_password,

            },
        )
        .map(|_| conn.last_insert_id())?;

    if last_insert_id > 0 {
        Ok(last_insert_id)
    } else {
        Err(PersistenceError::Unknown)
    }
}

pub fn select_user_by_id(pool: &Pool, id: u64) -> Result<UserEntity, PersistenceError> {
    let mut conn = pool.get_conn()?;

    // 使用参数化查询以避免SQL注入风险
    let user = conn
        .exec_map(
            "SELECT id, username, email FROM user WHERE id = :id limit 1",
            params! {"id" => id},
            |(id, username, email)| UserEntity {
                id,
                username,
                email,
                password: "".to_string(),
            },
        )?
        .into_iter()
        .next();
    match user {
        None => Err(PersistenceError::Unknown),
        Some(user) => Ok(user),
    }
}

pub fn select_user_by_email(pool: &Pool, email: String) -> Result<UserEntity, PersistenceError> {
    let mut conn = pool.get_conn()?;

    // 使用参数化查询以避免SQL注入风险
    let user = conn
        .exec_map(
            "SELECT id, username, email, password FROM user WHERE email = :email limit 1",
            params! {"email" => email},
            |(id, username, email, password)| UserEntity {
                id,
                username,
                email,
                password,
            },
        )?
        .into_iter()
        .next();
    match user {
        None => Err(PersistenceError::Unknown),
        Some(user) => Ok(user),
    }
}

pub fn update_user_by_id(
    pool: &Pool,
    id: u64,
    update_form: UserUpdateForm,
) -> Result<(), PersistenceError> {
    let mut conn = pool.get_conn()?;

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
    let mut query = "UPDATE user SET".to_string();
    let mut params = vec![];

    for (_i, (field, value)) in fields_values.iter().enumerate() {
        query.push_str(&format!(" {} = :{},", field, field));
        params.push((format!("{}", field), Value::from(value)));
    }
    // 移除最后一个逗号
    query.pop(); // Remove the last comma

    query.push_str(" where id = :id");
    params.push(("id".to_string(), Value::from(id)));

    // 执行 SQL 查询
    let mut stmt = conn.prep(&query).unwrap();
    let result = conn.exec_drop(&stmt, Params::from(params))?;
    log::info!("update user sql: {:?}", result);

    Ok(())
    // if last_insert_id > 0 {
    //     log::info!("update user success");
    //     Ok(last_insert_id)
    // } else {
    //     log::info!("update user error");
    //     Err(PersistenceError::Unknown)
    // }
}

pub fn insert_article(
    pool: &Pool,
    create_form: ArticleCreateForm,
    user_id: u64,
) -> Result<u64, PersistenceError> {
    let mut conn = pool.get_conn()?;

    let last_insert_id = conn.exec_drop(
        "insert into article(title, slug, description, body, created_at, updated_at, user_id) \
    values (:title, :slug, :description, :body, :created_at, :updated_at, :user_id)",
        params! {
            "title" => &create_form.title,
            "slug" => slugify!(&create_form.title),
            "description" => create_form.description,
            "body" => create_form.body,
            "created_at" => chrono::Utc::now().to_rfc3339(),
            "updated_at" => chrono::Utc::now().to_rfc3339(),
            "user_id" => user_id,
        },
    )
    .map(|_| conn.last_insert_id())?;

    if last_insert_id > 0 {
        Ok(last_insert_id)
    } else {
        Err(PersistenceError::Unknown)
    }
}

pub fn select_article_by_id(pool: &Pool, id: u64) -> Result<ArticleEntity, PersistenceError> {
    let mut conn = pool.get_conn()?;

    // 使用参数化查询以避免SQL注入风险
    let article = conn
        .exec_map(
            "SELECT id, title, slug, description, body, created_at, updated_at FROM article WHERE id = :id limit 1",
            params! {"id" => id},
            |(id, title, slug, description, body, created_at, updated_at)| {
            ArticleEntity {
                id,
                title,
                slug,
                description,
                body,
                created_at,
                updated_at,
            }
        })?
        .into_iter()
        .next();
    match article {
        None => Err(PersistenceError::Unknown),
        Some(article) => Ok(article),
    }
}