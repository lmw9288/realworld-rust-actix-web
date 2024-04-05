use crate::{models::TagsWrapper, persistence::tag::select_all_tag};
use actix_web::{get, web, Responder};
use sqlx::MySqlPool;

#[get("")]
pub async fn all_tags(pool: web::Data<MySqlPool>) -> actix_web::Result<impl Responder> {
    let tags = select_all_tag(&pool).await?;
    Ok(web::Json(TagsWrapper { tags }))
}
