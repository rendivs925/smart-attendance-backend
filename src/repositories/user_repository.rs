use crate::{
    config::database::{get_collection, USER_COL_NAME},
    models::user_model::User,
    types::requests::user::update_user_request::UpdateUserRequest,
};
use futures_util::stream::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId, to_document, Document};
use mongodb::{error::Result, Client, Collection};

pub struct UserRepository {
    pub collection: Collection<User>,
}

impl UserRepository {
    pub async fn new(client: &Client) -> Result<Self> {
        let collection = get_collection(client, USER_COL_NAME).await?;
        Ok(Self { collection })
    }

    pub async fn create_user(&self, user: &User) -> Result<User> {
        self.collection.insert_one(user).await?;
        Ok(User { ..user.clone() })
    }

    fn build_identifier_query(identifier: &str) -> Document {
        if let Ok(object_id) = ObjectId::parse_str(identifier) {
            doc! { "_id": object_id }
        } else if identifier.contains('@') {
            doc! { "email": identifier }
        } else {
            doc! { "phone_number": identifier }
        }
    }

    pub async fn find_user_by_identifier(&self, identifier: &str) -> Result<Option<User>> {
        let filter = Self::build_identifier_query(identifier);
        self.collection.find_one(filter).await
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        let cursor = self.collection.find(doc! {}).await?;
        let users: Vec<User> = cursor.try_collect().await?;
        Ok(users)
    }

    pub async fn update_user(
        &self,
        identifier: &str,
        user: UpdateUserRequest,
    ) -> Result<UpdateUserRequest> {
        let filter = Self::build_identifier_query(identifier);
        let update_doc = to_document(&user)?;

        self.collection
            .update_one(filter, doc! { "$set": update_doc })
            .await?;

        Ok(user)
    }

    pub async fn delete_user(&self, identifier: &str) -> Result<()> {
        let filter = Self::build_identifier_query(identifier);
        self.collection.delete_one(filter).await?;
        Ok(())
    }
}
