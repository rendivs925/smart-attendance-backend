use actix_web::web;
use dotenv::dotenv;
use shuttle_actix_web::ShuttleActixWeb;
use smart_attendance_backend::{
    config::database::create_unique_indexes,
    routes::{auth_routes::configure_auth_routes, user_routes::configure_user_routes},
};

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    dotenv().ok();

    match create_unique_indexes().await {
        Ok(_) => println!("Unique indexes created successfully."),
        Err(e) => eprintln!("Error creating indexes: {}", e),
    }

    let config = move |cfg: &mut web::ServiceConfig| {
        configure_user_routes(cfg);
        configure_auth_routes(cfg);
    };

    Ok(config.into())
}
