use crate::types::user::{
    defaults::{default_role, default_status, default_subscription_plan},
    permissions::Permission,
    role::Role,
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
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,

    #[validate(length(
        min = 3,
        max = 50,
        message = "Username must be between 3 and 50 characters"
    ))]
    pub name: String,

    #[validate(email(message = "Invalid email format"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[validate(length(
        min = 10,
        max = 15,
        message = "Phone number must be between 10 and 15 digits"
    ))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    #[serde(default = "default_role")]
    pub role: Role,

    #[serde(default)]
    pub permissions: HashSet<Permission>,

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
