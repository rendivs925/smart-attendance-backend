use actix_web::web;
use dotenv::dotenv;
use shuttle_actix_web::ShuttleActixWeb;
use smart_attendance_backend::{
    routes::organization_routes::configure_organization_routes,
    routes::user_routes::configure_user_routes,
    setup::{database::setup_database, services::setup_services},
};

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    dotenv().ok();
    println!("🚀 Starting Smart Attendance Backend...");

    let client = setup_database().await;
    let (user_service, organization_service) = setup_services(&client).await;

    let config = move |cfg: &mut web::ServiceConfig| {
        configure_user_routes(cfg, user_service.clone());
        configure_organization_routes(cfg, organization_service.clone());
    };

    println!("✅ Application started successfully");
    Ok(config.into())
}
