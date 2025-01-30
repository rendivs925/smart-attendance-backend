use crate::types::role::Role;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub _id: Option<ObjectId>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub nim: Option<String>,
    pub nidn: Option<String>,
    pub role: Role,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
