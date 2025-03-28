use crate::{
    constants::BCRYPT_COST,
    models::user_model::User,
    repositories::user_repository::UserRepository,
    utils::auth_utils::{generate_jwt, verify_password},
};
use anyhow::{anyhow, Context, Result};
use bcrypt::hash;
use std::sync::Arc;

pub struct UserService {
    pub user_repository: Arc<UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn authenticate_user(
        &self,
        identifier: &str,
        password: &str,
    ) -> Result<(User, String)> {
        let user = self
            .user_repository
            .find_user_by_email_or_phone_number(identifier)
            .await?
            .ok_or_else(|| anyhow!("Invalid credentials"))?;

        if !verify_password(password, &user.password) {
            return Err(anyhow!("Invalid credentials"));
        }

        let user_id = user
            ._id
            .as_ref()
            .map(|id| id.to_string())
            .ok_or_else(|| anyhow!("User ID is missing"))?;

        let token = generate_jwt(&user_id, &user.role, user.email.as_deref())
            .map_err(|e| anyhow!(e))
            .context("Failed to generate JWT")?;

        Ok((user, token))
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<User>> {
        self.user_repository
            .find_user_by_email(&email)
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn find_user_by_phone_number(&self, phone_number: &str) -> Result<Option<User>> {
        self.user_repository
            .find_user_by_phone_number(&phone_number)
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn create_user(&self, mut user: User) -> Result<User> {
        let cost: u32 = *BCRYPT_COST;

        let hashed_password = hash(&user.password, cost)
            .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?;

        user.password = hashed_password;

        self.user_repository
            .create_user(&user)
            .await
            .map_err(anyhow::Error::from)
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
