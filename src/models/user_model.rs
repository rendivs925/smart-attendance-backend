use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use validator::Validate;

use crate::types::user::{
    permissions::Permission, role::Role, subscription::SubscriptionPlan, user_status::UserStatus,
};

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,

    #[validate(length(
        min = 3,
        max = 50,
        message = "Username must be between 3 and 50 characters"
    ))]
    pub username: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password_hash: String,

    pub role: Role,

    pub permissions: HashSet<Permission>,

    #[serde(default)]
    pub organization_ids: HashSet<ObjectId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_plan: Option<SubscriptionPlan>,

    pub status: UserStatus,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
