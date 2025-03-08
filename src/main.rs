use actix_web::web;
use dotenv::dotenv;
use shuttle_actix_web::ShuttleActixWeb;
use smart_attendance_backend::{
    config::database::{connect_to_database, create_unique_indexes},
    repositories::{attendance_repository::AttendanceRepository, user_repository::UserRepository},
    routes::user_routes::configure_user_routes,
    services::user_service::UserService,
    setup::{database::setup_database, services::setup_services},
};
use std::sync::Arc;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    dotenv().ok();
    println!("🚀 Starting Smart Attendance Backend...");

    let client = setup_database().await;
    let user_service = setup_services(&client).await;

    let config = move |cfg: &mut web::ServiceConfig| {
        configure_user_routes(cfg, user_service.clone());
    };

    println!("✅ Application started successfully");
    Ok(config.into())
}
