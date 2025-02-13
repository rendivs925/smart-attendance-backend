use crate::{
    models::user_model::User,
    repositories::user_repository::UserRepository,
    types::user_response::UserResponse,
    utils::auth_utils::{generate_jwt, verify_password},
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

    pub async fn login_user(
        &self,
        nim: &str,
        password: &str,
    ) -> Result<Option<(UserResponse, String)>> {
        let user = self
            .user_repository
            .find_user_by_nim(nim)
            .await
            .map_err(anyhow::Error::from)?;

        if let Some(user) = user {
            if verify_password(password, &user.password) {
                let token = generate_jwt(
                    &user._id.as_ref().unwrap().to_string(),
                    &user.role,
                    user.email.as_deref(),
                )
                .unwrap();

                let user_response = UserResponse {
                    _id: user._id,
                    nim: user.nim,
                    role: user.role,
                    email: user.email,
                    username: user.username,
                    created_at: user.created_at,
                    nidn: user.nidn,
                    phone: user.phone,
                    updated_at: user.updated_at,
                };

                return Ok(Some((user_response, token)));
            }
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
