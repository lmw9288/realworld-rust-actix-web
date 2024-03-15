use bcrypt::{hash, verify};

pub fn encrypt_password(password: String) -> String {
    // 生成密码的哈希值
    let hashed_password = match hash(password, bcrypt::DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => panic!("Failed to hash password"),
    };
    hashed_password
}

pub fn verify_password(password: String, hash: &str) -> bool {
    let is_verified = verify(password, hash).unwrap_or(false);
    is_verified
}