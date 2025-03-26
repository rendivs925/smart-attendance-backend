use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::types::organization::organization_limit::OrganizationLimits;

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

    pub email: String,

    pub owner_id: ObjectId,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(url(message = "Invalid logo URL"))]
    pub logo_url: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub limits: OrganizationLimits,
}
