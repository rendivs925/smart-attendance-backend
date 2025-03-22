use crate::{
    models::user_model::User, repositories::user_repository::UserRepository,
    utils::auth_utils::generate_jwt,
};
use anyhow::Result;
use std::sync::Arc;

pub struct UserService {
    user_repository: Arc<UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn create_user(&self, user: User) -> Result<User> {
        self.user_repository
            .create_user(&user)
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn login_user(&self, user_id: &str) -> Result<Option<(User, String)>> {
        let user = self
            .user_repository
            .find_user_by_id(user_id)
            .await
            .map_err(anyhow::Error::from)?;

        if let Some(user) = user {
            let token = generate_jwt(
                &user._id.as_ref().unwrap().to_hex(),
                &user.role,
                Some(&user.email),
            )
            .unwrap();

            let user_response = User {
                _id: user._id,
                username: user.username,
                email: user.email,
                password_hash: user.password_hash,
                role: user.role,
                permissions: user.permissions.clone(),
                organization_ids: user.organization_ids.clone(),
                subscription_plan: user.subscription_plan.clone(),
                status: user.status,
                created_at: user.created_at,
                updated_at: user.updated_at,
            };

            return Ok(Some((user_response, token)));
        }
        Ok(None)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        self.user_repository
            .get_all_users()
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn get_user(&self, user_id: &str) -> Result<Option<User>> {
        self.user_repository
            .find_user_by_id(user_id)
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn update_user(&self, user_id: &str, user: User) -> Result<User> {
        self.user_repository
            .update_user(user_id, &user)
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn delete_user(&self, user_id: &str) -> Result<()> {
        self.user_repository
            .delete_user(user_id)
            .await
            .map_err(anyhow::Error::from)
    }
}
