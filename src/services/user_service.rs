use crate::config::database::connect_to_database;
use crate::models::user_model::User;
use crate::types::api_response::ApiResponse;
use crate::types::role::Role;
use crate::utils::auth_utils::hash_password;
use bson::oid::ObjectId;
use chrono::Utc;
use futures_util::TryStreamExt;
use mongodb::bson::doc;
use validator::Validate;

impl User {
    pub async fn create_user(user: &User) -> Result<User, ApiResponse<String>> {
        user.validate().map_err(|e| {
            ApiResponse::new(400, format!("Validation error: {}", e).to_string(), None)
        })?;

        match user.role {
            Role::Admin => {
                if user.email.is_none() {
                    return Err(ApiResponse::new(
                        400,
                        "Email is required for Admin".to_string(),
                        None,
                    ));
                }
            }
            Role::Teacher => {
                if user.nidn.is_none() {
                    return Err(ApiResponse::new(
                        400,
                        "NIDN is required for Teacher".to_string(),
                        None,
                    ));
                }
            }
            Role::Student => {
                if user.nim.is_none() {
                    return Err(ApiResponse::new(
                        400,
                        "NIM is required for Student".to_string(),
                        None,
                    ));
                }
            }
        }

        let hashed_password = hash_password(&user.password).map_err(|e| {
            ApiResponse::new(
                500,
                format!("Error hashing password: {}", e).to_string(),
                None,
            )
        })?;

        let client = connect_to_database().await.map_err(|e| {
            ApiResponse::new(
                500,
                format!("Error connecting to database: {}", e).to_string(),
                None,
            )
        })?;

        let collection = client
            .database("smart-attendance")
            .collection::<User>("users");

        let new_user = User {
            _id: Some(ObjectId::new()),
            password: hashed_password,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
            ..user.clone()
        };

        let result = collection.insert_one(&new_user).await;

        match result {
            Ok(_) => Ok(new_user),
            Err(e) => Err(ApiResponse::new(
                500,
                format!("Error inserting user: {}", e).to_string(),
                None,
            )),
        }
    }

    pub async fn get_all_users() -> Result<Vec<User>, ApiResponse<String>> {
        let client = connect_to_database().await.map_err(|e| {
            ApiResponse::new(
                500,
                format!("Error connecting to database: {}", e).to_string(),
                None,
            )
        })?;
        let collection = client
            .database("smart-attendance")
            .collection::<User>("users");

        let cursor = collection.find(doc! {}).await;

        match cursor {
            Ok(mut cursor) => {
                let mut users = Vec::new();
                while let Ok(Some(user)) = cursor.try_next().await {
                    users.push(user);
                }
                Ok(users)
            }
            Err(_) => Err(ApiResponse::new(
                500,
                "Error fetching users".to_string(),
                None,
            )),
        }
    }

    pub async fn find_user_by_id(user_id: &str) -> Result<Option<User>, ApiResponse<String>> {
        let client = connect_to_database().await.map_err(|e| {
            ApiResponse::new(
                500,
                format!("Error connecting to database: {}", e).to_string(),
                None,
            )
        })?;
        let collection = client
            .database("smart-attendance")
            .collection::<User>("users");

        let object_id = ObjectId::parse_str(user_id)
            .map_err(|_| ApiResponse::new(400, "Invalid ObjectId".to_string(), None))?;
        let filter = doc! { "_id": object_id };

        match collection.find_one(filter).await {
            Ok(user) => Ok(user),
            Err(_) => Err(ApiResponse::new(
                500,
                "Error fetching user by ID".to_string(),
                None,
            )),
        }
    }

    pub async fn update_user(user_id: &str, user_data: &User) -> Result<User, ApiResponse<String>> {
        user_data.validate().map_err(|e| {
            ApiResponse::new(400, format!("Validation error: {}", e).to_string(), None)
        })?;

        let client = connect_to_database().await.map_err(|e| {
            ApiResponse::new(
                500,
                format!("Error connecting to database: {}", e).to_string(),
                None,
            )
        })?;
        let collection = client
            .database("smart-attendance")
            .collection::<User>("users");

        let object_id = ObjectId::parse_str(user_id)
            .map_err(|_| ApiResponse::new(400, "Invalid ObjectId".to_string(), None))?;
        let filter = doc! { "_id": object_id };

        let update = doc! {
            "$set": {
                "username": &user_data.username,
                "email": &user_data.email,
                "role": &user_data.role.to_string(),
                "phone": &user_data.phone,
                "nim": &user_data.nim,
                "nidn": &user_data.nidn,
            }
        };

        match collection.update_one(filter, update).await {
            Ok(_) => Ok(user_data.clone()),
            Err(_) => Err(ApiResponse::new(
                500,
                "Error updating user".to_string(),
                None,
            )),
        }
    }

    pub async fn delete_user(user_id: &str) -> Result<(), ApiResponse<String>> {
        let client = connect_to_database().await.map_err(|e| {
            ApiResponse::new(
                500,
                format!("Error connecting to database: {}", e).to_string(),
                None,
            )
        })?;
        let collection = client
            .database("smart-attendance")
            .collection::<User>("users");

        let object_id = ObjectId::parse_str(user_id)
            .map_err(|_| ApiResponse::new(400, "Invalid ObjectId".to_string(), None))?;
        let filter = doc! { "_id": object_id };

        match collection.delete_one(filter).await {
            Ok(_) => Ok(()),
            Err(_) => Err(ApiResponse::new(
                500,
                "Error deleting user".to_string(),
                None,
            )),
        }
    }

    pub async fn find_user_by_nim(nim: &str) -> Result<Option<User>, ApiResponse<String>> {
        let client = connect_to_database().await.map_err(|e| {
            ApiResponse::new(
                500,
                format!("Error connecting to database: {}", e).to_string(),
                None,
            )
        })?;
        let collection = client
            .database("smart-attendance")
            .collection::<User>("users");

        let filter = doc! { "nim": nim };

        match collection.find_one(filter).await {
            Ok(Some(user)) => Ok(Some(user)),
            Ok(None) => Ok(None),
            Err(err) => Err(ApiResponse::new(
                500,
                "Error fetching user by NIM".to_string(),
                Some(format!("Database error: {}", err)),
            )),
        }
    }

    pub fn verify_password(password: &str, password_hash: &str) -> bool {
        bcrypt::verify(password, password_hash).unwrap_or(false)
    }
}
