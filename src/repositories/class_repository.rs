use crate::config::database::{get_collection, CLASS_COL_NAME};
use crate::models::class_model::Class;
use bson::{doc, oid::ObjectId};
use futures_util::TryStreamExt;
use mongodb::{error::Result, Client, Collection};

pub struct ClassRepository {
    collection: Collection<Class>,
}

impl ClassRepository {
    pub async fn new(client: &Client) -> Result<Self> {
        let collection = get_collection(client, CLASS_COL_NAME).await?;
        Ok(Self { collection })
    }

    pub async fn create_class(&self, class: &Class) -> Result<Class> {
        let new_class = Class {
            _id: Some(ObjectId::new()),
            ..class.clone()
        };

        self.collection.insert_one(&new_class).await?;
        Ok(new_class)
    }

    pub async fn get_all_classes(&self) -> Result<Vec<Class>> {
        let mut cursor = self.collection.find(doc! {}).await?;
        let mut classes = Vec::new();

        while let Some(class) = cursor.try_next().await? {
            classes.push(class);
        }

        Ok(classes)
    }

    pub async fn find_class_by_id(&self, class_id: &str) -> Result<Option<Class>> {
        let object_id = ObjectId::parse_str(class_id).unwrap();
        let class = self.collection.find_one(doc! { "_id": object_id }).await?;
        Ok(class)
    }

    pub async fn update_class(&self, class_id: &str, class_data: &Class) -> Result<Class> {
        let object_id = ObjectId::parse_str(class_id).unwrap();

        let update_doc = doc! {
            "$set": {
                "class_name": &class_data.class_name,
                "teacher_id": &class_data.teacher_id,
            }
        };

        self.collection
            .update_one(doc! { "_id": object_id }, update_doc)
            .await?;
        Ok(class_data.clone())
    }

    pub async fn delete_class(&self, class_id: &str) -> Result<()> {
        let object_id = ObjectId::parse_str(class_id).unwrap();
        self.collection
            .delete_one(doc! { "_id": object_id })
            .await?;
        Ok(())
    }
}
