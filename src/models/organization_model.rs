use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::subscription::SubscriptionPlan;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Organization {
    pub _id: ObjectId,
    pub name: String,
    pub owner_id: ObjectId,
    pub subscription_plan: SubscriptionPlan,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub limits: OrganizationLimits,
}

impl Organization {
    pub fn new(id: ObjectId, name: String, owner_id: ObjectId, plan: SubscriptionPlan) -> Self {
        Self {
            _id: id,
            name,
            owner_id,
            subscription_plan: plan.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            limits: plan.default_limits(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrganizationLimits {
    pub max_users: u32,
    pub max_attendance_logs: u32,
}
