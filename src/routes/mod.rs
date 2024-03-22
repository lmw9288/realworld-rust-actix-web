pub mod users;
pub mod articles;

use std::ops::Add;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use actix_web::{error, get, post, put, web, Responder, Result};
use jsonwebtoken::{EncodingKey, Header};
use realworld_rust_actix_web::SessionState;
use sqlx::{MySql, MySqlPool, Pool};

use crate::models::{
    Claims, UserLogin, UserRegistryForm, UserResponse, UserUpdateForm, UserWrapper,
};
use crate::persistence::{insert_user, select_user_by_email, select_user_by_id, update_user_by_id};
use crate::utils::verify_password;

