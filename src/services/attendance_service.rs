use crate::config::database::connect_to_database;
use crate::models::attendance_model::Attendance;
use crate::types::api_response::ApiResponse;
use bson::oid::ObjectId;
use bson::to_bson;
use futures_util::TryStreamExt;
use mongodb::bson::doc;

impl Attendance {
    pub async fn create_attendance(
        attendance: Attendance,
    ) -> Result<Attendance, ApiResponse<String>> {
        let client = connect_to_database().await.map_err(|e| {
            ApiResponse::new(500, format!("Error connecting to database: {}", e), None)
        })?;

        let collection = client
            .database("smart-attendance")
            .collection::<Attendance>("attendances");

        let result = collection.insert_one(&attendance).await;

        match result {
            Ok(_) => Ok(attendance),
            Err(e) => Err(ApiResponse::new(
                500,
                format!("Error inserting attendance: {}", e),
                None,
            )),
        }
    }

    pub async fn get_all_attendances() -> Result<Vec<Attendance>, ApiResponse<String>> {
        let client = connect_to_database().await.map_err(|e| {
            ApiResponse::new(500, format!("Error connecting to database: {}", e), None)
        })?;
        let collection = client
            .database("smart-attendance")
            .collection::<Attendance>("attendances");

        let cursor = collection.find(doc! {}).await;

        match cursor {
            Ok(mut cursor) => {
                let mut attendances = Vec::new();
                while let Ok(Some(attendance)) = cursor.try_next().await {
                    attendances.push(attendance);
                }
                Ok(attendances)
            }
            Err(_) => Err(ApiResponse::new(
                500,
                "Error fetching attendances".to_string(),
                None,
            )),
        }
    }

    pub async fn find_attendance_by_id(
        attendance_id: &str,
    ) -> Result<Option<Attendance>, ApiResponse<String>> {
        let client = connect_to_database().await.map_err(|e| {
            ApiResponse::new(500, format!("Error connecting to database: {}", e), None)
        })?;
        let collection = client
            .database("smart-attendance")
            .collection::<Attendance>("attendances");

        let object_id = ObjectId::parse_str(attendance_id)
            .map_err(|_| ApiResponse::new(400, "Invalid ObjectId".to_string(), None))?;
        let filter = doc! { "_id": object_id };

        match collection.find_one(filter).await {
            Ok(attendance) => Ok(attendance),
            Err(_) => Err(ApiResponse::new(
                500,
                "Error fetching attendance by ID".to_string(),
                None,
            )),
        }
    }

    pub async fn update_attendance(
        attendance_id: &str,
        attendance_data: &Attendance,
    ) -> Result<Attendance, ApiResponse<String>> {
        let client = connect_to_database().await.map_err(|e| {
            ApiResponse::new(500, format!("Error connecting to database: {}", e), None)
        })?;
        let collection = client
            .database("smart-attendance")
            .collection::<Attendance>("attendances");

        let object_id = ObjectId::parse_str(attendance_id)
            .map_err(|_| ApiResponse::new(400, "Invalid ObjectId".to_string(), None))?;
        let filter = doc! { "_id": object_id };

        let update = doc! {
            "$set": {
                "student_id": &attendance_data.student_id,
                "class_id": &attendance_data.class_id,
                "teacher_id": &attendance_data.teacher_id,
                "date": &attendance_data.date,
                "status": to_bson(&attendance_data.status)
                 .map_err(|_| ApiResponse::new(400, "Error converting status to BSON".to_string(), None))?,
            }
        };

        match collection.update_one(filter, update).await {
            Ok(_) => Ok(attendance_data.clone()),
            Err(_) => Err(ApiResponse::new(
                500,
                "Error updating attendance".to_string(),
                None,
            )),
        }
    }

    pub async fn delete_attendance(attendance_id: &str) -> Result<(), ApiResponse<String>> {
        let client = connect_to_database().await.map_err(|e| {
            ApiResponse::new(500, format!("Error connecting to database: {}", e), None)
        })?;
        let collection = client
            .database("smart-attendance")
            .collection::<Attendance>("attendances");

        let object_id = ObjectId::parse_str(attendance_id)
            .map_err(|_| ApiResponse::new(400, "Invalid ObjectId".to_string(), None))?;
        let filter = doc! { "_id": object_id };

        match collection.delete_one(filter).await {
            Ok(_) => Ok(()),
            Err(_) => Err(ApiResponse::new(
                500,
                "Error deleting attendance".to_string(),
                None,
            )),
        }
    }
}
