use super::{UserEntity, UserResponse};


pub fn to_author(user: UserEntity) -> UserResponse {
    UserResponse {
        username: user.username,
        email: user.email,
        token: None,
        bio: None,
        image: user.image,
    }
}
