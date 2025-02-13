use crate::{
    constants::COOKIE_NAME,
    models::user_model::User,
    services::user_service::UserService,
    types::api_response::ApiResponse,
    utils::{api_utils::create_response, auth_utils::create_http_only_cookie},
};
use actix_web::{
    cookie::{time::Duration, Cookie, SameSite},
    web, HttpRequest, HttpResponse, Responder,
};
use std::sync::Arc;

pub async fn create_user_handler(
    user_service: web::Data<Arc<UserService>>,
    user: web::Json<User>,
) -> impl Responder {
    match user_service.create_user(user.into_inner()).await {
        Ok(new_user) => create_response(201, "User registered successfully", Some(new_user)),
        Err(err) => create_response(500, "Failed to register user", Some(err.to_string())),
    }
}

pub async fn login_user_handler(
    user_service: web::Data<Arc<UserService>>,
    credentials: web::Json<User>,
) -> HttpResponse {
    let credentials_data = credentials.into_inner();
    let nim = credentials_data.nim.unwrap_or_default();

    match user_service
        .login_user(&nim, &credentials_data.password)
        .await
    {
        Ok(Some((user_response, token))) => {
            let cookie = create_http_only_cookie(token);
            HttpResponse::Ok().cookie(cookie).json(ApiResponse::new(
                200,
                "Login successful!".to_string(),
                Some(user_response),
            ))
        }
        Ok(None) => create_response::<String>(401, "Invalid credentials", None),
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

pub async fn update_user_handler(
    user_service: web::Data<Arc<UserService>>,
    user_id: web::Path<String>,
    user: web::Json<User>,
) -> HttpResponse {
    match user_service.update_user(&user_id, user.into_inner()).await {
        Ok(updated_user) => create_response(200, "User updated successfully", Some(updated_user)),
        Err(err) => create_response(500, "Error updating user", Some(err.to_string())),
    }
}

pub async fn delete_user_handler(
    user_service: web::Data<Arc<UserService>>,
    user_id: web::Path<String>,
) -> HttpResponse {
    match user_service.delete_user(&user_id).await {
        Ok(_) => create_response::<String>(204, "User deleted successfully", None),
        Err(err) => create_response(500, "Error deleting user", Some(err.to_string())),
    }
}
