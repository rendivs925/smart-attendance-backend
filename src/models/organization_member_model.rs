use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

use crate::types::models::user::role::Role;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct OrganizationMember {
    pub organization_id: ObjectId,

    #[validate(length(
        min = 3,
        max = 50,
        message = "Name must be between 3 and 50 characters"
    ))]
    pub name: String,

    pub role: Role,

    #[serde(default)]
    pub identifiers: HashMap<String, String>,

    #[serde(default = "Utc::now")]
    pub joined_at: DateTime<Utc>,
}
