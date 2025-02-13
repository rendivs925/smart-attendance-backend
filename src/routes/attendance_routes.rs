use crate::config::cors::configure_cors;
use crate::handlers::attendance_handler::{
    create_attendance_handler, delete_attendance_handler, get_all_attendances_handler,
    get_attendance_handler, update_attendance_handler,
};
use crate::services::attendance_service::AttendanceService;
use actix_web::web;
use std::sync::Arc;

pub fn configure_attendance_routes(
    cfg: &mut web::ServiceConfig,
    attendance_service: Arc<AttendanceService>,
) {
    let attendance_service_data = web::Data::new(attendance_service);

    cfg.service(
        web::scope("/attendances")
            .app_data(attendance_service_data.clone())
            .route("/new", web::post().to(create_attendance_handler))
            .route("/all", web::get().to(get_all_attendances_handler))
            .route("/{id}", web::get().to(get_attendance_handler))
            .route("/{id}", web::put().to(update_attendance_handler))
            .route("/{id}", web::delete().to(delete_attendance_handler))
            .wrap(configure_cors()),
    );
}
