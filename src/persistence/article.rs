use chrono::Utc;
use sqlx::MySqlPool;

use crate::models::{
    ArticleCreateForm, ArticleEntity, ArticleFavoriteEntity, ArticleQuery, ArticleUpdateForm,
};

use slugify::slugify;

use super::PersistenceError;

pub async fn insert_article(
    pool: &MySqlPool,
    create_form: ArticleCreateForm,
    user_id: i64,
) -> Result<u64, PersistenceError> {
    // let mut conn = pool.get_conn()?;

    let result = sqlx::query!(
        "insert into article(title, slug, description, body, created_at, updated_at, tag_list, user_id) values (?, ?, ?, ?, ?, ?, ?, ?)",
        create_form.title,
        slugify::slugify!(&create_form.title),
        create_form.description,
        create_form.body,
        Utc::now().naive_utc(),
        Utc::now().naive_utc(),
        serde_json::to_string(&create_form.tag_list).unwrap_or("[]".to_string()),
        user_id
    ).execute(pool).await?;

    for tag in create_form.tag_list {
        sqlx::query!(
            "insert into tag(name, article_id, user_id) values (?, ?, ?)",
            tag,
            result.last_insert_id(),
            user_id
        )
        .execute(pool)
        .await?;
    }

    if result.last_insert_id() > 0 {
        Ok(result.last_insert_id())
    } else {
        Err(PersistenceError::Unknown)
    }
}

pub async fn select_articles_by_query(
    pool: &MySqlPool,
    query: ArticleQuery,
) -> Result<Vec<ArticleEntity>, PersistenceError> {
    let mut sql = "SELECT a.id, a.title, a.slug, a.description, a.body, a.created_at, a.updated_at, a.tag_list, a.user_id, count(*) as favorites_count 
    FROM article a left join article_favorite af on a.id = af.article_id ".to_string();

    let mut values = vec![];
    if query.tag.is_some() {
        if values.len() == 0 {
            sql.push_str(" where ");
        }
        sql.push_str(" a.id in (select article_id from tag where name = ?) ");
        values.push(query.tag.unwrap());
    }
    if query.favorited.is_some() {
        if values.len() == 0 {
            sql.push_str(" where ");
        }
        sql.push_str(" a.id in (select article_id from article_favorite af join user on af.user_id = user.id where user.username = ?) ");
        values.push(query.favorited.unwrap());
    }
    sql.push_str("group by a.id order by a.id desc limit ?, ?");
    values.push(query.offset.unwrap_or(0).to_string());
    values.push(query.limit.unwrap_or(20).to_string());
    let mut query_as = sqlx::query_as(sql.as_str());
    for v in values {
        query_as = query_as.bind(v);
    }
    let result = query_as.fetch_all(pool).await;
    match result {
        Ok(articles) => Ok(articles),
        Err(e) => {
            log::error!("select article by query error: {}", e);
            Err(PersistenceError::Unknown)
        }
    }
}

//
pub async fn select_article_by_id(
    pool: &MySqlPool,
    id: u64,
) -> Result<ArticleEntity, PersistenceError> {
    // let mut conn = pool.get_conn()?;

    // 使用参数化查询以避免SQL注入风险
    let result = sqlx::query_as!(ArticleEntity,
        "SELECT a.id, a.title, a.slug, a.description, a.body, a.created_at, a.updated_at, a.tag_list, a.user_id, count(*) as favorites_count
        FROM article a left join article_favorite af on a.id = af.article_id
        WHERE a.id = ? group by a.id order by a.id desc limit 1",
        (id)
        )
        .fetch_one(pool)
        .await;
    // let tags = sqlx::query_scalar!("SELECT name FROM tag WHERE article_id = ?", (id))
    //     .fetch_all(pool)
    //     .await?;
    match result {
        Ok(article) => Ok(article),
        Err(e) => {
            log::error!("select article by id error: {}", e);
            Err(PersistenceError::Unknown)
        }
    }
}

pub async fn select_article_by_slug(
    pool: &MySqlPool,
    slug: String,
) -> Result<ArticleEntity, PersistenceError> {
    // let mut conn = pool.get_conn()?;

    // 使用参数化查询以避免SQL注入风险
    let result = sqlx::query_as!(ArticleEntity,
        "SELECT a.id, a.title, a.slug, a.description, a.body, a.created_at, a.updated_at, a.tag_list, a.user_id, count(*) as favorites_count
        FROM article a left join article_favorite af on a.id = af.article_id
        WHERE a.slug = ? group by a.id order by a.id desc limit 1",
        (slug)
        )
        .fetch_one(pool)
        .await;
    // let tags = sqlx::query_scalar!("SELECT name FROM tag WHERE article_id = ?", (id))
    //     .fetch_all(pool)
    //     .await?;
    match result {
        Ok(article) => Ok(article),
        Err(e) => {
            log::error!("select article by slug error: {}", e);
            Err(PersistenceError::Unknown)
        }
    }
}

pub async fn update_article_by_slug(
    pool: &MySqlPool,
    user_id: i64,
    slug: String,
    update_form: ArticleUpdateForm,
) -> Result<(), PersistenceError> {
    let mut sql = "update article ".to_string();

    let mut values = vec![];
    if update_form.title.is_some() {
        if values.len() == 0 {
            sql.push_str(" set ");
        }
        sql.push_str("title = ?, slug = ?");
        let title = update_form.title.unwrap();
        let title2 = title.clone();
        values.push(title);
        values.push(slugify::slugify!(&title2));
    }
    if update_form.body.is_some() {
        if values.len() == 0 {
            sql.push_str(" set ");
        }
        sql.push_str("body = ?,");
        values.push(update_form.body.unwrap());
    }
    if update_form.description.is_some() {
        if values.len() == 0 {
            sql.push_str(" set ");
        }
        sql.push_str("description = ?,");
        values.push(update_form.description.unwrap());
    }
    sql = sql[..sql.len() - 1].to_string();

    sql.push_str(" where slug = ? and user_id = ?");
    values.push(slug);
    values.push(user_id.to_string());
    log::info!("update article sql: {}", sql);

    let mut query_as = sqlx::query(sql.as_str());
    for v in values {
        query_as = query_as.bind(v);
    }
    let result = query_as.execute(pool).await?;
    if result.rows_affected() > 0 {
        Ok(())
    } else {
        Err(PersistenceError::Unknown)
    }
}

pub async fn delete_article_by_slug(
    pool: &MySqlPool,
    user_id: i64,
    slug: String,
) -> Result<(), PersistenceError> {
    let result = sqlx::query!(
        "delete from article where slug = ? and user_id = ?",
        slug,
        user_id
    )
    .execute(pool)
    .await?;
    if result.rows_affected() > 0 {
        Ok(())
    } else {
        Err(PersistenceError::Unknown)
    }
}

pub async fn insert_article_favorite(
    pool: &MySqlPool,
    user_id: i64,
    article_id: i64,
) -> Result<i64, PersistenceError> {
    let result = sqlx::query!(
        "insert article_favorite(user_id, article_id) values (?, ?)",
        user_id,
        article_id
    )
    .execute(pool)
    .await?;

    if result.last_insert_id() > 0 {
        Ok(result.last_insert_id() as i64)
    } else {
        Err(PersistenceError::Unknown)
    }
}

pub async fn select_article_favorite_by_user_id_and_article_id(
    pool: &MySqlPool,
    user_id: i64,
    article_id: i64,
) -> Result<Option<ArticleFavoriteEntity>, PersistenceError> {
    let result = sqlx::query_as!(
        ArticleFavoriteEntity,
        "select user_id, article_id from article_favorite where user_id = ? and article_id = ?",
        user_id,
        article_id
    )
    .fetch_optional(pool)
    .await;
    match result {
        Ok(favorite) => Ok(favorite),
        Err(e) => {
            log::error!(
                "select article favorite by user id and article id error: {}",
                e
            );
            Err(PersistenceError::Unknown)
        }
    }
}

pub async fn delete_article_favorite(
    pool: &MySqlPool,
    user_id: i64,
    article_id: i64,
) -> Result<(), PersistenceError> {
    let result = sqlx::query!(
        "delete from article_favorite where user_id = ? and article_id = ?",
        user_id,
        article_id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() > 0 {
        Ok(())
    } else {
        Err(PersistenceError::Unknown)
    }
}
