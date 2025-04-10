use crate::{
    config::database::{get_collection, USER_COL_NAME},
    models::user_model::User,
};
use futures_util::stream::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId, to_document};
use mongodb::{error::Result, Client, Collection};

pub struct OrganizationMemberRepository {
    pub collection: Collection<User>,
}

impl OrganizationMemberRepository {
    pub async fn new(client: &Client) -> Result<Self> {
        let collection = get_collection(client, USER_COL_NAME).await?;
        Ok(Self { collection })
    }

    pub async fn create_user(&self, user: &User) -> Result<User> {
        self.collection.insert_one(user).await?;
        Ok(User { ..user.clone() })
    }

    pub async fn find_user_by_id(&self, user_id: &str) -> Result<Option<User>> {
        self.collection.find_one(doc! { "_id": user_id }).await
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<User>> {
        self.collection.find_one(doc! { "email": email }).await
    }

    pub async fn find_user_by_phone_number(&self, phone_number: &str) -> Result<Option<User>> {
        self.collection
            .find_one(doc! { "phone_number": phone_number })
            .await
    }

    pub async fn find_user_by_email_or_phone_number(
        &self,
        identifier: &str,
    ) -> Result<Option<User>> {
        let filter = doc! {
            "$or": [
                { "email": identifier },
                { "phone_number": identifier }
            ]
        };

        self.collection.find_one(filter).await
    }
    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        let cursor = self.collection.find(doc! {}).await?;
        let users: Vec<User> = cursor.try_collect().await?;
        Ok(users)
    }

    pub async fn update_user(&self, user_id: &str, user: &User) -> Result<User> {
        let object_id = ObjectId::parse_str(user_id).unwrap();
        let update_doc = to_document(user)?;

        self.collection
            .update_one(doc! { "_id": object_id }, doc! { "$set": update_doc })
            .await?;

        Ok(user.clone())
    }

    pub async fn delete_user(&self, user_id: &str) -> Result<()> {
        self.collection.delete_one(doc! { "_id": user_id }).await?;
        Ok(())
    }
}
