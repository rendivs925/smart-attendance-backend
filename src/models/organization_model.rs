use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::types::{
    organization::organization_limit::OrganizationLimits, user::subscription::SubscriptionPlan,
};

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct Organization {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,

    #[validate(length(
        min = 3,
        max = 100,
        message = "Organization name must be between 3 and 100 characters"
    ))]
    pub name: String,

    pub owner_id: ObjectId,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password_hash: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(url(message = "Invalid logo URL"))]
    pub logo_url: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub limits: OrganizationLimits,
}

impl Organization {
    pub fn new(
        id: ObjectId,
        name: String,
        owner_id: ObjectId,
        password_hash: String,
        plan: SubscriptionPlan,
    ) -> Self {
        Self {
            _id: Some(id),
            name,
            owner_id,
            password_hash,
            logo_url: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            limits: plan.default_limits(),
        }
    }
}
