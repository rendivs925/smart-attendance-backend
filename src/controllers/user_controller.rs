use crate::constants::COOKIE_NAME;
use crate::types::user_response::UserResponse;
use crate::utils::auth_utils::verify_password;
use crate::{
    models::user_model::User,
    types::api_response::ApiResponse,
    utils::api_utils::create_response,
    utils::auth_utils::{create_http_only_cookie, generate_jwt},
};
use actix_web::{
    cookie::time::Duration,
    cookie::{Cookie, SameSite},
    web, HttpRequest, HttpResponse, Responder,
};

pub async fn create_user_handler(user: web::Json<User>) -> impl Responder {
    match User::create_user(&user.into_inner()).await {
        Ok(new_user) => create_response(201, "User created successfully", Some(new_user)),
        Err(err) => {
            eprintln!("Error creating user: {}", err);
            create_response(500, "Failed to create user", Some(err))
        }
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

pub async fn login_user_handler(credentials: web::Json<User>) -> HttpResponse {
    let credentials_data = credentials.into_inner();
    let nim = credentials_data.nim.clone().unwrap_or_default();

    match User::find_user_by_nim(&nim).await {
        Ok(Some(user)) => {
            if !verify_password(&credentials_data.password, &user.password) {
                return create_response::<String>(401, "Invalid credentials", None);
            }

            let jwt_token = match &user._id {
                Some(user_id) => {
                    generate_jwt(&user_id.to_string(), &user.role, user.email.as_deref())
                        .map_err(|_| "Failed to generate JWT")
                }
                None => Err("User ID is missing"),
            };

            match jwt_token {
                Ok(token) => {
                    let cookie = create_http_only_cookie(token);

                    let user_response = UserResponse {
                        _id: user._id,
                        nim: user.nim,
                        role: user.role,
                        email: user.email,
                        username: user.username,
                        created_at: user.created_at,
                        nidn: user.nidn,
                        phone: user.phone,
                        updated_at: user.updated_at,
                    };

                    HttpResponse::Ok().cookie(cookie).json(ApiResponse::new(
                        200,
                        "Login successful!".to_string(),
                        Some(user_response),
                    ))
                }
                Err(err) => create_response::<String>(500, err, None),
            }
        }
        Ok(None) => create_response::<String>(404, "User not found", None),
        Err(err) => create_response(500, "Error finding user", Some(err)),
    }
}

pub async fn get_all_users_handler() -> HttpResponse {
    match User::get_all_users().await {
        Ok(users) => create_response(200, "Users fetched successfully", Some(users)),
        Err(err) => {
            eprintln!("Error fetching users: {}", err);
            create_response(500, "Error fetching users", Some(err))
        }
    }
}

pub async fn get_user_handler(user_id: web::Path<String>) -> HttpResponse {
    match User::find_user_by_id(&user_id).await {
        Ok(Some(user)) => create_response(200, "User found successfully", Some(user)),
        Ok(None) => create_response::<String>(404, "User not found", None),
        Err(err) => create_response(500, "Error fetching user", Some(err)),
    }
}

pub async fn update_user_handler(
    user_id: web::Path<String>,
    user: web::Json<User>,
) -> HttpResponse {
    match User::update_user(&user_id, &user.into_inner()).await {
        Ok(updated_user) => create_response(200, "User updated successfully", Some(updated_user)),
        Err(err) => {
            eprintln!("Error updating user: {}", err);
            create_response(500, "Error updating user", Some(err))
        }
    }
}

pub async fn delete_user_handler(user_id: web::Path<String>) -> HttpResponse {
    match User::delete_user(&user_id).await {
        Ok(_) => create_response::<String>(204, "User deleted successfully", None),
        Err(err) => {
            eprintln!("Error deleting user: {}", err);
            create_response(500, "Error deleting user", Some(err))
        }
    }
}
