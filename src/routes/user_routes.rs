use crate::{
    config::cors::configure_cors,
    controllers::user_controller::{
        create_user_handler, delete_user_handler, get_all_users_handler, get_user_handler,
        login_user_handler, update_user_handler,
    },
};
use actix_web::web;

pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/new", web::post().to(create_user_handler))
            .route("/all", web::get().to(get_all_users_handler))
            .route("/{id}", web::get().to(get_user_handler))
            .route("/{id}", web::put().to(update_user_handler))
            .route("/{id}", web::delete().to(delete_user_handler)),
    );

    cfg.service(
        web::scope("/login")
            .route("", web::post().to(login_user_handler))
            .wrap(configure_cors()),
    );
}
