use crate::types::responses::api_response::ApiResponse;
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
use std::sync::Arc;

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
                Some(&user.email).unwrap().as_deref(),
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
    let cookie = Cookie::build(&*COOKIE_NAME, "")
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
