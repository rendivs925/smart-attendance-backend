use crate::handlers::user_handler::{
    create_user_handler, delete_user_handler, get_all_users_handler, get_user_handler,
    update_user_handler,
};
use crate::services::user_service::UserService;
use actix_web::web;
use std::sync::Arc;

pub fn configure_user_routes(
    cfg: &mut web::ServiceConfig,
    user_service_data: web::Data<Arc<UserService>>,
) {
    cfg.service(
        web::scope("/users")
            .app_data(user_service_data)
            .route("/new", web::post().to(create_user_handler))
            .route("/all", web::get().to(get_all_users_handler))
            .route("/{identifier}", web::get().to(get_user_handler))
            .route("/{identifier}", web::put().to(update_user_handler))
            .route("/{identifier}", web::delete().to(delete_user_handler)),
    );
}
