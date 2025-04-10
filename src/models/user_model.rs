use crate::types::user::{
    defaults::{default_status, default_subscription_plan},
    subscription::SubscriptionPlan,
    user_status::UserStatus,
};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct User {
    #[validate(length(
        min = 3,
        max = 50,
        message = "Username must be between 3 and 50 characters"
    ))]
    pub name: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    #[serde(default)]
    pub organization_ids: HashSet<ObjectId>,

    #[serde(default)]
    pub owned_organizations: u32,

    #[serde(default = "default_subscription_plan")]
    pub subscription_plan: SubscriptionPlan,

    #[serde(default = "default_status")]
    pub status: UserStatus,

    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,

    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
}
