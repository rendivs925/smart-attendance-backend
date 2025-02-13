use crate::types::role::Role;
use crate::utils::auth_utils::validate_phone_number;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct User {
    pub _id: Option<ObjectId>,

    #[validate(length(
        min = 3,
        max = 50,
        message = "Username must be between 3 and 50 characters"
    ))]
    pub username: Option<String>,

    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,

    #[validate(custom(function = "validate_phone_number"))]
    pub phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[validate(length(equal = 10, message = "NIM must be exactly 10 characters"))]
    pub nim: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[validate(length(equal = 10, message = "NIDN must be exactly 10 characters"))]
    pub nidn: Option<String>,

    pub role: Role,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
