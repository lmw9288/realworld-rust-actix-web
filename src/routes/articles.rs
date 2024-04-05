use crate::models::{
    ArticleCreateForm, ArticleEntity, ArticleQuery, ArticleResponse, ArticleUpdateForm,
    ArticleWrapper, ArticlesWrapper, UserEntity, UserResponse,
};
use crate::persistence::article::{
    delete_article_by_slug, delete_article_favorite, insert_article, insert_article_favorite,
    select_article_by_id, select_article_by_slug,
    select_article_favorite_by_user_id_and_article_id, select_articles_by_query,
    update_article_by_slug,
};
use crate::persistence::user::select_user_by_id;
use actix_web::{delete, get, post, put, web, Responder};

use realworld_rust_actix_web::SessionState;
use sqlx::MySqlPool;

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

    log::info!("articles = {:?}", articles);

    let mut result_articles = vec![];
    for a in articles {
        // log::info!("article = {:?}", a);
        let user = select_user_by_id(&pool, a.user_id).await?;
        let article_favorite =
            select_article_favorite_by_user_id_and_article_id(&pool, a.user_id, a.id).await?;
        let favorited = if article_favorite.is_some() {
            true
        } else {
            false
        };
        result_articles.push(to_article_response(a, user, favorited))
    }

    Ok(web::Json(ArticlesWrapper::<ArticleResponse> {
        articles: result_articles,
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
        article: to_article_response(article, user, false),
    };
    log::info!("create_article: r = {:?}", r);

    Ok(web::Json(r))
}

#[delete("/{slug}")]
pub async fn delete_article(
    session_state: SessionState,
    pool: web::Data<MySqlPool>,
    path: web::Path<String>,
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
    path: web::Path<String>,
    data: web::Json<ArticleWrapper<ArticleUpdateForm>>,
) -> actix_web::Result<impl Responder> {
    let user_id = session_state.user_id;
    let slug = path.into_inner();
    let slug2 = slug.clone();
    let update_form = data.into_inner().article;
    update_article_by_slug(&pool, user_id, slug, update_form).await?;
    let article = select_article_by_slug(&pool, slug2).await?;

    let af = select_article_favorite_by_user_id_and_article_id(&pool, user_id, article.id).await?;
    let favorited = if af.is_some() { true } else { false };

    let user = select_user_by_id(&pool, user_id).await?;

    Ok(web::Json(ArticleWrapper {
        article: to_article_response(article, user, favorited),
    }))
}

//
#[get("/feed")]
pub async fn list_articles_feed(// session_state: SessionState,
    // pool: web::Data<MySqlPool>,
) -> actix_web::Result<impl Responder> {
    Ok(web::Json(ArticlesWrapper::<ArticleResponse> {
        articles: vec![],
        articles_count: 0,
    }))
}

//
#[get("/{slug}")]
pub async fn single_article(path: web::Path<String>) -> actix_web::Result<impl Responder> {
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
    path: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let slug = path.into_inner();
    let user_id = session_state.user_id;

    let article = select_article_by_slug(&pool, slug).await?;
    let user = select_user_by_id(&pool, user_id).await?;
    insert_article_favorite(&pool, user_id, article.id).await?;
    // log::info!()
    Ok(web::Json(ArticleWrapper {
        article: to_article_response(article, user, true),
    }))
}

#[delete("/{slug}/favorite")]
pub async fn unfavorite_article(
    session_state: SessionState,
    pool: web::Data<MySqlPool>,
    path: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let slug = path.into_inner();
    let user_id = session_state.user_id;

    let article = select_article_by_slug(&pool, slug).await?;
    let user = select_user_by_id(&pool, user_id).await?;
    delete_article_favorite(&pool, user_id, article.id).await?;

    Ok(web::Json(ArticleWrapper {
        article: to_article_response(article, user, false),
    }))
}

fn to_article_response(
    article: ArticleEntity,
    user: UserEntity,
    favorited: bool,
) -> ArticleResponse {
    let mut tag_list = serde_json::from_str(&article.tag_list).unwrap_or(Vec::<String>::new());
    tag_list.sort();
    ArticleResponse {
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
        favorites_count: article.favorites_count,
        favorited,
        tag_list,
        author: to_author(user),
    }
}

fn to_author(user: UserEntity) -> UserResponse {
    UserResponse {
        username: user.username,
        email: user.email,
        token: None,
        bio: None,
        image: None,
    }
}
