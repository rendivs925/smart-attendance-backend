use once_cell::sync::Lazy;
use std::env;

pub static BCRYPT_COST: Lazy<u32> = Lazy::new(|| {
    env::var("BCRYPT_COST")
        .expect("BCRYPT_COST must be set")
        .parse()
        .expect("BCRYPT_COST must be a valid u32")
});
pub static CLIENT_ID: Lazy<String> =
    Lazy::new(|| env::var("CLIENT_ID").expect("CLIENT_ID must be set"));
pub static CLIENT_SECRET: Lazy<String> =
    Lazy::new(|| env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set"));
pub static REDIRECT_URI: Lazy<String> =
    Lazy::new(|| env::var("REDIRECT_URI").expect("REDIRECT_URI must be set"));
pub static JWT_SECRET_KEY: Lazy<String> =
    Lazy::new(|| env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set"));
pub static COOKIE_NAME: Lazy<String> =
    Lazy::new(|| env::var("COOKIE_NAME").unwrap_or_else(|_| "user_jwt_token".to_string()));
