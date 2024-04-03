use crate::models::{
    ArticleCreateForm, ArticleQuery, ArticleResponse, ArticleUpdateForm, ArticleWrapper,
    ArticlesWrapper, CommentResponse, CommentsWrapper, UserResponse,
};
use crate::persistence::{delete_article_by_slug, insert_article, select_article_by_id, select_articles_by_query, select_user_by_id};
use actix_web::{delete, get, post, put, web, Responder};
use chrono::Utc;
use realworld_rust_actix_web::SessionState;
use sqlx::{query_builder, MySqlPool, QueryBuilder};
use std::ops::Deref;
use actix_web::web::delete;

//
#[get("")]
pub async fn list_articles(
    pool: web::Data<MySqlPool>,
    query: web::Query<ArticleQuery>,
) -> actix_web::Result<impl Responder> {
    log::info!("list_articles query = {:?}", query);

    let query = query.into_inner();

    let articles = select_articles_by_query(&pool, query).await?;
    // let user = select_user_by_id(&pool, user_id).await?;

    Ok(web::Json(ArticlesWrapper::<ArticleResponse> {
        articles: articles
            .into_iter()
            .map(|a| ArticleResponse {
                title: a.title,
                slug: a.slug,
                description: a.description,
                body: a.body,
                created_at: a.created_at.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                updated_at: a.updated_at.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                favorited: false,
                favorites_count: 0,
                tag_list: serde_json::from_str(&a.tag_list).unwrap_or(vec![]),
                author: UserResponse {
                    username: "".to_string(),
                    email: "".to_string(),
                    token: None,
                    bio: None,
                    image: None,
                },
            })
            .collect(),
        articles_count: 0,
    }))
}

//
#[post("")]
pub async fn create_article(
    session_state: SessionState,
    pool: web::Data<MySqlPool>,
    data: web::Json<ArticleWrapper<ArticleCreateForm>>,
) -> actix_web::Result<impl Responder> {
    log::info!("create_article data = {:?}", data);
    let user_id = session_state.user_id;
    let article = data.into_inner().article;
    // let tagList = article.clone().tagList;
    let last_insert_id = insert_article(&pool, article, user_id).await?;
    let article = select_article_by_id(&pool, last_insert_id).await?;
    let user = select_user_by_id(&pool, user_id).await?;

    // let tz_offset = FixedOffset::east(8 * 3600);
    // let t = tz_offset.from_local_datetime(&article.created_at).unwrap().to_rfc3339();
    // log::info!("t = {}", t);

    let r = ArticleWrapper {
        article: ArticleResponse {
            title: article.title,
            slug: article.slug,
            description: article.description,
            body: article.body,
            created_at: article
                .created_at
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
            updated_at: article
                .updated_at
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
            favorites_count: 0,
            favorited: false,
            tag_list: serde_json::from_str(&article.tag_list).unwrap_or(vec![]),
            author: UserResponse {
                username: user.username,
                email: user.email,
                token: None,
                bio: None,
                image: None,
            },
        },
    };
    log::info!("create_article: r = {:?}", r);

    Ok(web::Json(r))
}

#[delete("/{slug}")]
pub async fn delete_article(
    session_state: SessionState,
    pool: web::Data<MySqlPool>,
    path: web::Path<(String)>,
) -> actix_web::Result<impl Responder> {
    let user_id = session_state.user_id;
    let slug = path.into_inner();
    log::info!("delete_article: slug: {:?}", slug);
    
    delete_article_by_slug(&pool, user_id, slug).await?;
    Ok(web::Json(()))
}

#[put("/{slug}")]
pub async fn update_article(
    session_state: SessionState,
    pool: web::Data<MySqlPool>,
    path: web::Path<(String)>,
    data: web::Json<ArticleWrapper<ArticleUpdateForm>>,
) -> actix_web::Result<impl Responder> {
    let user_id = session_state.user_id;
    let slug = path.into_inner();
    // let update_form = data.article;
    // update_article_by_slug(&pool, user_id, path.0, data.0.article);
    Ok(web::Json(ArticleWrapper {
        article: ArticleResponse {
            title: "".to_string(),
            slug,
            body: "".to_string(),
            description: "".to_string(),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
            favorites_count: 0,
            favorited: false,
            tag_list: vec![],
            author: UserResponse {
                username: "".to_string(),
                email: "".to_string(),
                token: None,
                bio: None,
                image: None,
            },
        },
    }))
}

//
#[get("/feed")]
pub async fn list_articles_feed(
    session_state: SessionState,
    pool: web::Data<MySqlPool>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(ArticlesWrapper::<ArticleResponse> {
        articles: vec![],
        articles_count: 0,
    }))
}

//
#[get("/{slug}")]
pub async fn single_article(path: web::Path<(String)>) -> actix_web::Result<impl Responder> {
    log::info!("single_article: path: {:?}", path);

    Ok(web::Json(ArticleWrapper {
        article: ArticleResponse {
            title: "".to_string(),
            slug: "".to_string(),
            description: "".to_string(),
            body: "".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            favorites_count: 1,
            favorited: false,
            tag_list: vec![],
            author: UserResponse {
                username: "".to_string(),
                email: "".to_string(),
                token: None,
                bio: None,
                image: None,
            },
        },
    }))
}

#[post("/{slug}/favorite")]
pub async fn favorite_article(
    session_state: SessionState,
    pool: web::Data<MySqlPool>,
    path: web::Path<(String)>,
) -> actix_web::Result<impl Responder> {
    let slug = path.into_inner();
    // log::info!()
    Ok(web::Json(ArticleWrapper {
        article: ArticleResponse {
            title: "".to_string(),
            slug: "".to_string(),
            description: "".to_string(),
            body: "".to_string(),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
            favorites_count: 1,
            favorited: true,
            tag_list: vec![],
            author: UserResponse {
                username: "".to_string(),
                email: "".to_string(),
                token: None,
                bio: None,
                image: None,
            },
        },
    }))
}

#[delete("/{slug}/favorite")]
pub async fn unfavorite_article(
    session_state: SessionState,
    pool: web::Data<MySqlPool>,
    path: web::Path<(String)>,
) -> actix_web::Result<impl Responder> {
    let slug = path.into_inner();
    Ok(web::Json(ArticleWrapper {
        article: ArticleResponse {
            title: "".to_string(),
            slug: "".to_string(),
            description: "".to_string(),
            body: "".to_string(),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
            favorites_count: 1,
            favorited: false,
            tag_list: vec![],
            author: UserResponse {
                username: "".to_string(),
                email: "".to_string(),
                token: None,
                bio: None,
                image: None,
            },
        },
    }))
}
