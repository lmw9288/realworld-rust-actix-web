use chrono::Utc;
use sqlx::{Execute, MySqlPool, QueryBuilder};

use crate::{
    models::user::{UserEntity, UserFollowEntity, UserUpdateForm},
    utils::encrypt_password,
};

use super::PersistenceError;

pub async fn insert_user(
    pool: &MySqlPool,
    username: String,
    email: String,
    password: String,
) -> Result<u64, PersistenceError> {
    let hash_password = encrypt_password(password);

    let result = sqlx::query!(
        "INSERT INTO user (created_at, updated_at, username, email, password) VALUES (?, ?, ?, ?, ?)",
        Utc::now().naive_utc(),
        Utc::now().naive_utc(),
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
        "SELECT id, username, email, password, image, bio FROM user WHERE id = ? limit 1",
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
        "SELECT id, username, email, password, image, bio FROM user WHERE email = ? limit 1",
        (email)
    )
    .fetch_one(pool)
    .await?;
    Ok(user)
}

pub async fn select_user_by_username(
    pool: &MySqlPool,
    username: String,
) -> Result<UserEntity, PersistenceError> {
    let user = sqlx::query_as!(
        UserEntity,
        "SELECT id, username, email, password, image, bio FROM user WHERE username = ? order by id desc limit 1",
        (username)
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
    // let() mut conn = pool.get_conn()?;

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

pub async fn select_follow_by_user(
    pool: &MySqlPool,
    follower_user_id: i64,
    followee_user_id: i64,
) -> Result<bool, PersistenceError> {
    let result = sqlx::query_as!(
        UserFollowEntity,
        "select followee_user_id, follower_user_id from user_follow where follower_user_id = ? and followee_user_id = ?",
        follower_user_id,
        followee_user_id)
        .fetch_optional(pool)
        .await?;
    if result.is_some() {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn insert_follow_by_user(
    pool: &MySqlPool,
    user_id: i64,
    followee_user_id: i64,
) -> Result<i64, PersistenceError> {
    let result = sqlx::query!(
        "insert user_follow(created_at, updated_at, follower_user_id, followee_user_id) values (?, ?, ?, ?)",
        chrono::Utc::now().naive_utc(),
        chrono::Utc::now().naive_utc(),
        user_id,
        followee_user_id
    )
    .execute(pool)
    .await?;

    if result.last_insert_id() > 0 {
        Ok(result.last_insert_id() as i64)
    } else {
        Err(PersistenceError::Unknown)
    }
}

pub async fn delete_follow_by_user(
    pool: &MySqlPool,
    user_id: i64,
    followee_user_id: i64,
) -> Result<(), PersistenceError> {
    let result = sqlx::query!(
        "delete from user_follow where follower_user_id = ? and followee_user_id = ?",
        user_id,
        followee_user_id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() > 0 {
        Ok(())
    } else {
        Err(PersistenceError::Unknown)
    }
}
