use crate::controllers::attendance_controller::{
    create_attendance_handler, delete_attendance_handler, get_all_attendances_handler,
    get_attendance_handler, update_attendance_handler,
};
use actix_web::web;

pub fn configure_attendance_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/attendances")
            .route("/new", web::post().to(create_attendance_handler))
            .route("/all", web::get().to(get_all_attendances_handler))
            .route("/{id}", web::get().to(get_attendance_handler))
            .route("/{id}", web::put().to(update_attendance_handler))
            .route("/{id}", web::delete().to(delete_attendance_handler)),
    );
}
