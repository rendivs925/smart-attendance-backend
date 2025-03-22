use crate::config::cors::configure_cors;
use crate::handlers::user_handler::{
    create_user_handler, delete_user_handler, get_all_users_handler, get_user_handler,
    jwt_login_handler, logout_user_handler, update_user_handler,
};
use crate::services::user_service::UserService;
use actix_web::web;
use std::sync::Arc;

pub fn configure_user_routes(cfg: &mut web::ServiceConfig, user_service: Arc<UserService>) {
    let user_service_data = web::Data::new(user_service);

    cfg.service(
        web::scope("/users")
            .app_data(user_service_data.clone())
            .route("/new", web::post().to(create_user_handler))
            .route("/all", web::get().to(get_all_users_handler))
            .route("/{id}", web::get().to(get_user_handler))
            .route("/{id}", web::put().to(update_user_handler))
            .route("/{id}", web::delete().to(delete_user_handler)),
    );

    cfg.service(
        web::scope("/login")
            .app_data(user_service_data.clone())
            .route("", web::post().to(jwt_login_handler))
            .wrap(configure_cors()),
    );

    cfg.service(
        web::scope("/logout")
            .app_data(user_service_data)
            .route("", web::delete().to(logout_user_handler))
            .wrap(configure_cors()),
    );
}
