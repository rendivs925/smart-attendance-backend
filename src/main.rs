use actix_web::web;
use dotenv::dotenv;
use env_logger;
use log::info;
use shuttle_actix_web::ShuttleActixWeb;
use smart_attendance_backend::{
    routes::organization_routes::configure_organization_routes,
    routes::user_routes::configure_user_routes,
    setup::{database::setup_database, services::setup_services},
};
use std;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "debug");
    if env_logger::try_init().is_err() {
        eprintln!("Logger already initialized");
    }

    info!("🚀 Starting Smart Attendance Backend...");

    let client = setup_database().await;
    let (user_service, organization_service) = setup_services(&client).await;

    let config = move |cfg: &mut web::ServiceConfig| {
        configure_user_routes(cfg, user_service.clone());
        configure_organization_routes(cfg, organization_service.clone());
    };

    info!("✅ Application started successfully");
    Ok(config.into())
}
