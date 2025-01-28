use crate::constants::{COOKIE_NAME, JWT_SECRET_ENV};
use crate::types::claims::Claims;
use crate::types::role::Role;
use actix_web::cookie::time::Duration as CookieDuration;
use actix_web::cookie::{Cookie, SameSite};
use bcrypt::{hash, DEFAULT_COST};
use chrono::{Duration as ChronoDuration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use phonenumber::{country, parse};
use std::env;
use validator::ValidationError;

pub fn create_http_only_cookie(token: String) -> Cookie<'static> {
    let max_age = CookieDuration::days(7);

    Cookie::build(COOKIE_NAME, token)
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/")
        .max_age(max_age)
        .finish()
}

pub fn generate_jwt(user_id: &str, role: &Role, email: Option<&str>) -> Result<String, String> {
    println!("Generating JWT for user_id: {}, role: {:?}", user_id, role);

    let secret_key = match get_jwt_secret() {
        Ok(key) => key,
        Err(e) => {
            println!("Error retrieving JWT secret: {}", e);
            return Err("Failed to get JWT secret".to_string());
        }
    };

    println!("JWT secret retrieved successfully");

    let expiration = Utc::now() + ChronoDuration::hours(24);
    let claims = Claims {
        _id: user_id.to_string(),
        role: role.to_string(),
        email: email.map(|e| e.to_string()),
        exp: expiration.timestamp() as usize,
    };

    println!("Claims created: {:?}", claims);

    let header = Header::new(Algorithm::HS256);

    match encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret_key.as_bytes()),
    ) {
        Ok(token) => {
            println!("JWT generated successfully");
            Ok(token)
        }
        Err(_) => {
            println!("Error generating JWT");
            Err("Error generating JWT".to_string())
        }
    }
}

pub fn verify_jwt(token: &str) -> Result<Claims, String> {
    let secret_key = get_jwt_secret()?;

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
    .map_err(|_| "Error verifying JWT".to_string())
}

pub fn get_jwt_secret() -> Result<String, String> {
    env::var(JWT_SECRET_ENV)
        .map_err(|_| format!("{} environment variable not set", JWT_SECRET_ENV).to_string())
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
