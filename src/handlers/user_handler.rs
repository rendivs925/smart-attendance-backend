use crate::types::responses::api_response::ApiResponse;
use crate::types::user::role::Role;
use crate::types::user::user_status::UserStatus;
use crate::{
    constants::COOKIE_NAME,
    models::user_model::User,
    repositories::user_repository::UserRepository,
    services::user_service::UserService,
    utils::{
        api_utils::create_response,
        auth_utils::{create_http_only_cookie, generate_jwt},
    },
};
use actix_web::{
    cookie::{time::Duration, Cookie, SameSite},
    web, HttpRequest, HttpResponse,
};
use bson::oid::ObjectId;
use chrono::Utc;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;

pub async fn oauth2_login_handler() -> HttpResponse {
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI must be set");

    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?response_type=code&client_id={}&redirect_uri={}&scope=email%20profile",
        client_id, redirect_uri
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

    let access_token = match tokens.get("access_token") {
        Some(token) => token.clone(),
        None => return create_response::<String>(500, "Missing access token", None),
    };

    let user_info = match fetch_user_info(&access_token).await {
        Ok(info) => info,
        Err(err) => return create_response(500, "Failed to fetch user info", Some(err)),
    };

    let google_id = user_info
        .get("id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let email = user_info
        .get("email")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let username = user_info
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

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

async fn exchange_code_for_token(code: &str) -> Result<HashMap<String, String>, String> {
    let client_id = env::var("CLIENT_ID").map_err(|_| "Missing CLIENT_ID".to_string())?;
    let client_secret =
        env::var("CLIENT_SECRET").map_err(|_| "Missing CLIENT_SECRET".to_string())?;
    let redirect_uri = env::var("REDIRECT_URI").map_err(|_| "Missing REDIRECT_URI".to_string())?;

    let client = Client::new();
    let params = [
        ("code", code),
        ("client_id", &client_id),
        ("client_secret", &client_secret),
        ("redirect_uri", &redirect_uri),
        ("grant_type", "authorization_code"),
    ];

    let response = client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await
        .map_err(|_| "OAuth2 token exchange request failed".to_string())?;

    let tokens = response
        .json::<HashMap<String, String>>()
        .await
        .map_err(|_| "Failed to parse OAuth2 token response".to_string())?;

    Ok(tokens)
}

async fn fetch_user_info(access_token: &str) -> Result<Value, String> {
    let client = Client::new();

    let response = client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|_| "Failed to send request to fetch user info".to_string())?;

    let user_info = response
        .json::<Value>()
        .await
        .map_err(|_| "Failed to parse user info response".to_string())?;

    Ok(user_info)
}

async fn register_new_user(
    user_repository: web::Data<Arc<UserRepository>>,
    username: String,
    email: String,
) -> Result<User, String> {
    let new_user = User {
        _id: Some(ObjectId::new()),
        username,
        email,
        password_hash: "".to_string(),
        role: Role::Admin,
        permissions: Default::default(),
        organization_ids: Default::default(),
        subscription_plan: None,
        status: UserStatus::Active,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    user_repository
        .create_user(&new_user)
        .await
        .map_err(|e| format!("Database error: {}", e))
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

pub async fn jwt_login_handler(
    user_repository: web::Data<Arc<UserRepository>>,
    credentials: web::Json<User>,
) -> HttpResponse {
    let user_id = match credentials._id.as_ref() {
        Some(id) => id.to_string(),
        None => return create_response::<String>(400, "User ID is required for login", None),
    };

    match user_repository.find_user_by_id(&user_id).await {
        Ok(Some(user)) => {
            let token = match generate_jwt(
                &user
                    ._id
                    .as_ref()
                    .map(|id| id.to_string())
                    .unwrap_or_default(),
                &user.role,
                Some(&user.email),
            ) {
                Ok(t) => t,
                Err(e) => {
                    return create_response::<String>(
                        500,
                        &format!("JWT generation failed: {}", e),
                        None,
                    )
                }
            };

            let cookie = create_http_only_cookie(token.clone());

            HttpResponse::Ok().cookie(cookie).json(ApiResponse::new(
                200,
                "Login successful!".to_string(),
                Some(user),
            ))
        }
        Ok(None) => create_response::<String>(401, "User not found", None),
        Err(err) => create_response(500, "Error logging in", Some(err.to_string())),
    }
}

pub async fn logout_user_handler(_req: HttpRequest) -> HttpResponse {
    let cookie = Cookie::build(COOKIE_NAME, "")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::None)
        .path("/")
        .max_age(Duration::new(0, 0))
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(ApiResponse::<String>::new(
            200,
            "Logout successful!".to_string(),
            None,
        ))
}

pub async fn get_all_users_handler(user_service: web::Data<Arc<UserService>>) -> HttpResponse {
    match user_service.get_all_users().await {
        Ok(users) => create_response(200, "Users fetched successfully", Some(users)),
        Err(err) => create_response(500, "Error fetching users", Some(err.to_string())),
    }
}

pub async fn get_user_handler(
    user_service: web::Data<Arc<UserService>>,
    user_id: web::Path<String>,
) -> HttpResponse {
    match user_service.get_user(&user_id).await {
        Ok(Some(user)) => create_response(200, "User found successfully", Some(user)),
        Ok(None) => create_response::<String>(404, "User not found", None),
        Err(err) => create_response(500, "Error fetching user", Some(err.to_string())),
    }
}

pub async fn create_user_handler(
    user_service: web::Data<Arc<UserService>>,
    user: web::Json<User>,
) -> HttpResponse {
    let new_user = user.into_inner();

    match user_service.create_user(new_user).await {
        Ok(new_user) => create_response(201, "User registered successfully", Some(new_user)),
        Err(err) => create_response(500, "Failed to register user", Some(err.to_string())),
    }
}

pub async fn update_user_handler(
    user_service: web::Data<Arc<UserService>>,
    user_id: web::Path<String>,
    user: web::Json<User>,
) -> HttpResponse {
    user_service
        .update_user(&user_id, user.into_inner())
        .await
        .map(|updated_user| create_response(200, "User updated successfully", Some(updated_user)))
        .unwrap_or_else(|err| create_response(500, "Error updating user", Some(err.to_string())))
}

pub async fn delete_user_handler(
    user_service: web::Data<Arc<UserService>>,
    user_id: web::Path<String>,
) -> HttpResponse {
    user_service
        .delete_user(&user_id)
        .await
        .map(|_| create_response::<String>(204, "User deleted successfully", None))
        .unwrap_or_else(|err| create_response(500, "Error deleting user", Some(err.to_string())))
}
