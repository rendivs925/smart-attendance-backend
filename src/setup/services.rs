use crate::{repositories::user_repository::UserRepository, services::user_service::UserService};
use std::sync::Arc;

pub async fn setup_services(client: &Arc<mongodb::Client>) -> Arc<UserService> {
    let user_repository = UserRepository::new(client)
        .await
        .expect("❌ Failed to initialize UserRepository");
    Arc::new(UserService::new(Arc::new(user_repository)))
}
