use crate::constants::{CLIENT_ID, REDIRECT_URI};
use crate::models::user_model::User;
use crate::repositories::user_repository::UserRepository;
use crate::services::oauth_service::{exchange_code_for_token, fetch_user_info, register_new_user};
use crate::types::responses::api_response::ApiResponse;
use crate::utils::api_utils::create_response;
use actix_web::{
    cookie::{time::Duration, Cookie, SameSite},
    web, HttpResponse,
};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn oauth2_login_handler() -> HttpResponse {
    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?response_type=code&client_id={}&redirect_uri={}&scope=email%20profile",
        *CLIENT_ID, *REDIRECT_URI
    );

    HttpResponse::Found()
        .append_header(("Location", auth_url))
        .finish()
}

pub async fn oauth2_callback_handler(
    user_repository: web::Data<Arc<UserRepository>>,
    query: web::Query<HashMap<String, String>>,
) -> HttpResponse {
    let code = match query.get("code") {
        Some(code) => code,
        None => return create_response::<String>(400, "Authorization code is missing", None),
    };

    let tokens = match exchange_code_for_token(code).await {
        Ok(tokens) => tokens,
        Err(err) => return create_response(500, "Failed to get OAuth2 tokens", Some(err)),
    };

    let access_token = tokens.get("access_token").cloned().unwrap_or_default();
    let user_info = match fetch_user_info(&access_token).await {
        Ok(info) => info,
        Err(err) => return create_response(500, "Failed to fetch user info", Some(err)),
    };

    let google_id = user_info["id"].as_str().unwrap_or("").to_string();
    let email = user_info["email"].as_str().unwrap_or("").to_string();
    let username = user_info["name"].as_str().unwrap_or("").to_string();

    if google_id.is_empty() || email.is_empty() || username.is_empty() {
        return create_response::<String>(500, "Invalid user data from OAuth2 provider", None);
    }

    match user_repository.find_user_by_id(&google_id).await {
        Ok(Some(user)) => build_login_response(user, &access_token),
        Ok(None) => match register_new_user(user_repository, username, email).await {
            Ok(new_user) => build_login_response(new_user, &access_token),
            Err(err) => create_response(500, "Failed to register user", Some(err.to_string())),
        },
        Err(err) => create_response(500, "Database error", Some(err.to_string())),
    }
}

fn build_login_response(user: User, access_token: &str) -> HttpResponse {
    let cookie = Cookie::build("oauth_token", access_token.to_string())
        .http_only(true)
        .secure(true)
        .same_site(SameSite::None)
        .path("/")
        .max_age(Duration::days(7))
        .finish();

    HttpResponse::Ok().cookie(cookie).json(ApiResponse::new(
        200,
        "Login successful!".to_string(),
        Some(user),
    ))
}
