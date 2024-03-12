#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct UserWrapper<T>
where
    T: serde::Serialize,
{
    pub user: T,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct UserRegistry {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct UserResponse {
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: Option<String>,
    pub image: Option<String>
}

