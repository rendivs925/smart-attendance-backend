use crate::config::cors::configure_cors;
use crate::controllers::user_controller::{login_user_handler, logout_user_handler};
use actix_web::web;

pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/login")
            .route("", web::post().to(login_user_handler))
            .wrap(configure_cors()),
    );

    cfg.service(
        web::scope("/logout")
            .route("", web::delete().to(logout_user_handler))
            .wrap(configure_cors()),
    );
}
