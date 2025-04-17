use crate::{
    models::user_model::User,
    repositories::user_repository::UserRepository,
    types::{
        models::user::defaults::default_status,
        requests::{
            auth::register_request::RegisterRequest, user::update_user_request::UpdateUserRequest,
        },
    },
    utils::auth_utils::{generate_jwt, hash_password, verify_password},
};
use anyhow::{anyhow, Context, Result};
use bson::oid::ObjectId;
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

    pub async fn authenticate_user(
        &self,
        identifier: &str,
        password: &str,
    ) -> Result<(User, String)> {
        let user = self
            .user_repository
            .find_user_by_identifier(identifier)
            .await?
            .ok_or_else(|| anyhow!("Invalid credentials"))?;

        if !verify_password(password, &user.password)
            .map_err(|err| anyhow!("Password verification failed: {:?}", err))?
        {
            return Err(anyhow!("Invalid credentials"));
        }

        let token = generate_jwt(&user.name, identifier)
            .map_err(|e| anyhow!(e))
            .context("Failed to generate JWT")?;

        Ok((user, token))
    }

    pub async fn create_user(&self, new_user: RegisterRequest) -> Result<User> {
        new_user.validate()?;

        if let Some(existing) = self
            .user_repository
            .find_user_by_identifier(new_user.email.as_str())
            .await?
        {
            return Err(anyhow::anyhow!(
                "User with the same email or phone number already exists: {}",
                existing.name
            ));
        }

        let hashed_password = hash_password(&new_user.password)
            .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?;

        let now = Utc::now();

        let user = User {
            _id: Some(ObjectId::new()),
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

    pub async fn get_user(&self, identifier: &str) -> Result<Option<User>> {
        self.user_repository
            .find_user_by_identifier(identifier)
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn update_user(
        &self,
        identifier: &str,
        user: UpdateUserRequest,
    ) -> Result<UpdateUserRequest> {
        self.user_repository
            .update_user(identifier, user)
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn delete_user(&self, identifier: &str) -> Result<()> {
        self.user_repository
            .delete_user(identifier)
            .await
            .map_err(anyhow::Error::from)
    }
}
