use crate::constants::{COOKIE_NAME, JWT_SECRET_KEY};
use crate::types::auth::claims::Claims;
use actix_web::cookie::time::Duration as CookieDuration;
use actix_web::cookie::{Cookie, SameSite};
use bcrypt::{hash, DEFAULT_COST};
use chrono::{Duration as ChronoDuration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::{error, info};
use phonenumber::{country, parse};
use validator::ValidationError;

pub fn create_http_only_cookie(token: String) -> Cookie<'static> {
    let max_age = CookieDuration::days(7);

    Cookie::build(COOKIE_NAME.as_str(), token)
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/")
        .max_age(max_age)
        .finish()
}

pub fn generate_jwt(name: &str, email: &str) -> Result<String, String> {
    let secret_key = JWT_SECRET_KEY.as_bytes();

    let expiration = Utc::now() + ChronoDuration::hours(24);
    let claims = Claims {
        name: name.to_owned(),
        email: email.to_owned(),
        exp: expiration.timestamp() as usize,
    };

    info!("Claims created successfully");

    let header = Header::new(Algorithm::HS256);

    encode(&header, &claims, &EncodingKey::from_secret(secret_key)).map_err(|e| {
        error!("Error generating JWT: {:?}", e);
        format!("JWT generation failed: {}", e)
    })
}

pub fn verify_jwt(token: &str) -> Result<Claims, String> {
    let secret_key = &JWT_SECRET_KEY;

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
    .map_err(|_| "Error verifying JWT".to_string())
}

pub fn hash_password(password: &str) -> Result<String, String> {
    hash(password, DEFAULT_COST).map_err(|_| "Error hashing password".to_string())
}

pub fn validate_phone_number(phone: &str) -> Result<(), ValidationError> {
    match parse(Some(country::ID), phone) {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("invalid_phone_number")),
    }
}

pub fn verify_password(password: &str, password_hash: &str) -> bool {
    bcrypt::verify(password, password_hash).unwrap_or(false)
}
