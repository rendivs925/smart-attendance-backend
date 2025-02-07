use crate::config::database::connect_to_database;
use crate::models::class_model::Class;
use crate::types::api_response::ApiResponse;
use bson::doc;
use bson::oid::ObjectId;
use futures_util::TryStreamExt;
use mongodb::Collection;

impl Class {
    pub async fn create_class(class: &Class) -> Result<Class, ApiResponse<String>> {
        let client = connect_to_database().await.map_err(|e| {
            ApiResponse::new(500, format!("Database connection error: {}", e), None)
        })?;

        let collection: Collection<Class> =
            client.database("smart-attendance").collection("classes");

        let new_class = Class {
            _id: Some(ObjectId::new()),
            ..class.clone()
        };

        collection
            .insert_one(&new_class)
            .await
            .map_err(|e| ApiResponse::new(500, format!("Failed to insert class: {}", e), None))?;

        Ok(new_class)
    }

    pub async fn get_all_classes() -> Result<Vec<Class>, ApiResponse<String>> {
        let client = connect_to_database().await.map_err(|e| {
            ApiResponse::new(500, format!("Database connection error: {}", e), None)
        })?;

        let collection: Collection<Class> =
            client.database("smart-attendance").collection("classes");

        let mut cursor = collection
            .find(doc! {})
            .await
            .map_err(|_| ApiResponse::new(500, "Failed to fetch classes".to_string(), None))?;

        let mut classes = Vec::new();
        while let Some(class) = cursor.try_next().await.map_err(|_| {
            ApiResponse::new(500, "Error iterating through classes".to_string(), None)
        })? {
            classes.push(class);
        }

        Ok(classes)
    }

    pub async fn find_class_by_id(class_id: &str) -> Result<Option<Class>, ApiResponse<String>> {
        let client = connect_to_database().await.map_err(|e| {
            ApiResponse::new(500, format!("Database connection error: {}", e), None)
        })?;

        let collection: Collection<Class> =
            client.database("smart-attendance").collection("classes");

        let object_id = ObjectId::parse_str(class_id)
            .map_err(|_| ApiResponse::new(400, "Invalid ObjectId".to_string(), None))?;

        collection
            .find_one(doc! { "_id": object_id })
            .await
            .map_err(|_| ApiResponse::new(500, "Error fetching class by ID".to_string(), None))
    }

    pub async fn update_class(
        class_id: &str,
        class_data: &Class,
    ) -> Result<Class, ApiResponse<String>> {
        let client = connect_to_database().await.map_err(|e| {
            ApiResponse::new(500, format!("Database connection error: {}", e), None)
        })?;

        let collection: Collection<Class> =
            client.database("smart-attendance").collection("classes");

        let object_id = ObjectId::parse_str(class_id)
            .map_err(|_| ApiResponse::new(400, "Invalid ObjectId".to_string(), None))?;

        let update_doc = doc! {
            "$set": {
                "class_name": &class_data.class_name,
                "teacher_id": &class_data.teacher_id,
            }
        };

        collection
            .update_one(doc! { "_id": object_id }, update_doc)
            .await
            .map_err(|_| ApiResponse::new(500, "Error updating class".to_string(), None))?;

        Ok(class_data.clone())
    }

    pub async fn delete_class(class_id: &str) -> Result<(), ApiResponse<String>> {
        let client = connect_to_database().await.map_err(|e| {
            ApiResponse::new(500, format!("Database connection error: {}", e), None)
        })?;

        let collection: Collection<Class> =
            client.database("smart-attendance").collection("classes");

        let object_id = ObjectId::parse_str(class_id)
            .map_err(|_| ApiResponse::new(400, "Invalid ObjectId".to_string(), None))?;

        collection
            .delete_one(doc! { "_id": object_id })
            .await
            .map_err(|_| ApiResponse::new(500, "Error deleting class".to_string(), None))?;

        Ok(())
    }
}
