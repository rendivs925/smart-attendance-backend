use crate::{
    constants::BCRYPT_COST,
    models::user_model::User,
    repositories::user_repository::UserRepository,
    types::{requests::register_request::RegisterRequest, user::defaults::default_status},
    utils::auth_utils::{generate_jwt, verify_password},
};
use anyhow::{anyhow, Context, Result};
use bcrypt::hash;
use chrono::Utc;
use std::{collections::HashSet, sync::Arc};
use validator::Validate;

pub struct UserService {
    pub user_repository: Arc<UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn authenticate_user(&self, email: &str, password: &str) -> Result<(User, String)> {
        let user = self
            .user_repository
            .find_user_by_email(email)
            .await?
            .ok_or_else(|| anyhow!("Invalid credentials"))?;

        if !verify_password(password, &user.password) {
            return Err(anyhow!("Invalid credentials"));
        }

        let token = generate_jwt(&user.name, email)
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

    pub async fn create_user(&self, new_user: RegisterRequest) -> Result<User> {
        new_user.validate()?;

        if let Some(existing) = self
            .user_repository
            .find_user_by_email(new_user.email.as_str())
            .await?
        {
            return Err(anyhow::anyhow!(
                "User with the same email or phone number already exists: {}",
                existing.name
            ));
        }

        let cost: u32 = *BCRYPT_COST;
        let hashed_password = hash(&new_user.password, cost)
            .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?;

        let now = Utc::now();

        let user = User {
            name: new_user.name,
            email: new_user.email,
            password: hashed_password,
            organization_ids: HashSet::new(),
            owned_organizations: 0,
            subscription_plan: new_user.subscription_plan,
            status: default_status(),
            created_at: now,
            updated_at: now,
        };

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
