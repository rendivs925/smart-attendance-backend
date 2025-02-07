use crate::controllers::class_controller::{
    create_class_handler, delete_class_handler, get_all_classes_handler, get_class_handler,
    update_class_handler,
};
use actix_web::web;

pub fn configure_class_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/classes")
            .route("/new", web::post().to(create_class_handler))
            .route("/all", web::get().to(get_all_classes_handler))
            .route("/{id}", web::get().to(get_class_handler))
            .route("/{id}", web::put().to(update_class_handler))
            .route("/{id}", web::delete().to(delete_class_handler)),
    );
}
