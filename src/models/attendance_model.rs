use crate::types::status::Status;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attendance {
    pub _id: Option<ObjectId>,
    pub student_id: ObjectId,
    pub class_id: ObjectId,
    pub teacher_id: ObjectId,
    pub date: String,
    pub status: Status,
}
