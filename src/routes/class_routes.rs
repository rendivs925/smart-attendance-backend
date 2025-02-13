use crate::handlers::class_handler::{
    create_class_handler, delete_class_handler, get_all_classes_handler, get_class_handler,
    update_class_handler,
};
use crate::services::class_service::ClassService;
use actix_web::web;
use std::sync::Arc;

pub fn configure_class_routes(cfg: &mut web::ServiceConfig, class_service: Arc<ClassService>) {
    let class_service_data = web::Data::new(class_service);

    cfg.service(
        web::scope("/classes")
            .app_data(class_service_data.clone())
            .route("/new", web::post().to(create_class_handler))
            .route("/all", web::get().to(get_all_classes_handler))
            .route("/{id}", web::get().to(get_class_handler))
            .route("/{id}", web::put().to(update_class_handler))
            .route("/{id}", web::delete().to(delete_class_handler)),
    );
}
