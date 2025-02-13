use actix_web::web;
use dotenv::dotenv;
use shuttle_actix_web::ShuttleActixWeb;
use smart_attendance_backend::{
    config::database::{connect_to_database, create_unique_indexes},
    repositories::{
        attendance_repository::AttendanceRepository, class_repository::ClassRepository,
        user_repository::UserRepository,
    },
    routes::{
        attendance_routes::configure_attendance_routes, class_routes::configure_class_routes,
        user_routes::configure_user_routes,
    },
    services::{
        attendance_service::AttendanceService, class_service::ClassService,
        user_service::UserService,
    },
};
use std::sync::Arc;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    dotenv().ok();
    println!("🚀 Starting Smart Attendance Backend...");

    let client = match connect_to_database().await {
        Ok(client) => {
            println!("✅ Connected to MongoDB");
            Arc::new(client)
        }
        Err(err) => {
            eprintln!("❌ Failed to connect to MongoDB: {}", err);
            std::process::exit(1);
        }
    };

    if let Err(err) = create_unique_indexes(&client).await {
        eprintln!("❌ Failed to create indexes: {}", err);
        std::process::exit(1);
    }
    println!("✅ Unique indexes created successfully");

    let user_repository = match UserRepository::new(&client).await {
        Ok(repo) => Arc::new(repo),
        Err(err) => {
            eprintln!("❌ Failed to initialize UserRepository: {}", err);
            std::process::exit(1);
        }
    };

    let class_repository = match ClassRepository::new(&client).await {
        Ok(repo) => Arc::new(repo),
        Err(err) => {
            eprintln!("❌ Failed to initialize ClassRepository: {}", err);
            std::process::exit(1);
        }
    };

    let attendance_repository = match AttendanceRepository::new(&client).await {
        Ok(repo) => Arc::new(repo),
        Err(err) => {
            eprintln!("❌ Failed to initialize AttendanceRepository: {}", err);
            std::process::exit(1);
        }
    };

    let user_service = Arc::new(UserService::new(user_repository.clone()));
    let class_service = Arc::new(ClassService::new(class_repository.clone()));
    let attendance_service = Arc::new(AttendanceService::new(attendance_repository.clone()));

    let config = move |cfg: &mut web::ServiceConfig| {
        configure_user_routes(cfg, user_service.clone());
        configure_class_routes(cfg, class_service.clone());
        configure_attendance_routes(cfg, attendance_service.clone());
    };

    println!("✅ Application started successfully");
    Ok(config.into())
}
