use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Class {
    pub _id: Option<ObjectId>,
    pub class_name: String,
    pub teacher_id: ObjectId,
}
