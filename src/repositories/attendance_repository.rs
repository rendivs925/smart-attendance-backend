use crate::config::database::get_collection;
use crate::models::attendance_model::Attendance;
use futures_util::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, to_document},
    error::Result,
    Client, Collection,
};

pub struct AttendanceRepository {
    collection: Collection<Attendance>,
}

impl AttendanceRepository {
    pub async fn new(client: &Client) -> Result<Self> {
        let collection = get_collection(client, "attendances").await?;
        Ok(Self { collection })
    }

    pub async fn create_attendance(&self, attendance: &Attendance) -> Result<Attendance> {
        let insert_result = self.collection.insert_one(attendance).await?;
        if let Some(inserted_id) = insert_result.inserted_id.as_object_id() {
            Ok(Attendance {
                _id: Some(inserted_id),
                ..attendance.clone()
            })
        } else {
            Err(mongodb::error::Error::custom(
                "Failed to retrieve inserted ObjectId",
            ))
        }
    }

    pub async fn get_all_attendances(&self) -> Result<Vec<Attendance>> {
        let cursor = self.collection.find(doc! {}).await?;
        let attendances: Vec<Attendance> = cursor.try_collect().await?;
        Ok(attendances)
    }

    pub async fn find_attendance_by_id(&self, attendance_id: &str) -> Result<Option<Attendance>> {
        let object_id = ObjectId::parse_str(attendance_id)
            .map_err(|_| mongodb::error::Error::custom("Invalid ObjectId format"))?;
        self.collection.find_one(doc! { "_id": object_id }).await
    }

    pub async fn update_attendance(
        &self,
        attendance_id: &str,
        attendance: &Attendance,
    ) -> Result<Attendance> {
        let object_id = ObjectId::parse_str(attendance_id)
            .map_err(|_| mongodb::error::Error::custom("Invalid ObjectId format"))?;
        let update_doc = to_document(attendance)?;

        self.collection
            .update_one(doc! { "_id": object_id }, doc! { "$set": update_doc })
            .await?;

        Ok(attendance.clone())
    }

    pub async fn delete_attendance(&self, attendance_id: &str) -> Result<()> {
        let object_id = ObjectId::parse_str(attendance_id)
            .map_err(|_| mongodb::error::Error::custom("Invalid ObjectId format"))?;
        self.collection
            .delete_one(doc! { "_id": object_id })
            .await?;
        Ok(())
    }
}
