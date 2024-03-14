use std::sync::Arc;
use actix_web::{post, web, Responder, Result};
use mysql::Pool;
use crate::models::{UserLogin, UserRegistryForm, UserResponse, UserWrapper};
use crate::persistence::{insert_user, select_user};

#[post("/users/login")]
pub async fn login_user(json: web::Json<UserWrapper<UserLogin>>,
                        data: web::Data<Pool>) -> Result<impl Responder> {
    // println!("login_user: {:?}", json);
    // let email = json.email;
    // let password = json.password;
    let UserLogin { email, password } = json.into_inner().user;


    Ok(web::Json(UserWrapper {
        user: UserResponse {
            username: "".to_owned(),
            email,
            bio: None,
            image: None,
        },
    }))
}

#[post("/users")]
pub async fn registry_user(
    json: web::Json<UserWrapper<UserRegistryForm>>,
    pool: web::Data<Pool>,
) -> Result<impl Responder> {
    let UserRegistryForm {
        username,
        email,
        password,
    } = json.into_inner().user;

    let user = web::block(move || {
        let last_insert_id = insert_user(&pool, username, email, password)?;
        select_user(&pool, last_insert_id)
    }).await??;


    Ok(web::Json(UserWrapper {
        user: UserResponse {
            username: user.username,
            email: user.email,
            bio: None,
            image: None,
        }
    }))
}
