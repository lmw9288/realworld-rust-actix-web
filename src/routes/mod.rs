use actix_web::{get, post, web, Responder, Result};

use crate::models::{UserLogin, UserRegistry, UserResponse, UserWrapper};

#[post("/login")]
pub async fn login_user(json: web::Json<UserWrapper<UserLogin>>) -> Result<impl Responder> {
    // println!("login_user: {:?}", json);
    // let email = json.email;
    // let password = json.password;
    let UserLogin { email, password } = json.into_inner().user;

    Ok(web::Json(UserWrapper {
        user: UserResponse {
            username: "".to_owned(),
            email,
            password,
            bio: None,
            image: None,
        },
    }))
}

#[post("")]
pub async fn registry_user(json: web::Json<UserWrapper<UserRegistry>>) -> Result<impl Responder> {
    let UserRegistry {
        username,
        email,
        password,
    } = json.into_inner().user;

    Ok(web::Json(UserWrapper {
        user: UserResponse {
            username,
            email,
            password,
            bio: None,
            image: None,
        },
    }))
}
