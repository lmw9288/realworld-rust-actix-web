use crate::models::TagsWrapper;
use actix_web::{web, Responder, get};

#[get("")]
pub async fn all_tags() -> actix_web::Result<impl Responder> {
    Ok(web::Json(TagsWrapper {
        tags: vec!["tag1".to_string(), "tag2".to_string()],
    }))
}
