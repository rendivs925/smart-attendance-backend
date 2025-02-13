use crate::models::user_model::User;
use mongodb::{
    bson::doc,
    error::Error as MongoError,
    options::{ClientOptions, IndexOptions},
    Client, Collection, IndexModel,
};
use std::env;

pub const DB_NAME: &str = "smart-attendance";
pub const USER_COL_NAME: &str = "users";
pub const CLASS_COL_NAME: &str = "classes";
pub const ATTENDANCE_COL_NAME: &str = "attendances";

pub async fn connect_to_database() -> Result<Client, MongoError> {
    let client_uri =
        env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

    let client_options = ClientOptions::parse(&client_uri).await?;
    Client::with_options(client_options)
}

pub async fn get_collection<T>(
    client: &Client,
    collection_name: &str,
) -> Result<Collection<T>, MongoError>
where
    T: serde::de::DeserializeOwned + serde::Serialize + Unpin + Send + Sync,
{
    Ok(client.database(DB_NAME).collection::<T>(collection_name))
}

async fn create_partial_unique_index(
    collection: &Collection<User>,
    field: &str,
) -> Result<(), MongoError> {
    let index = IndexModel::builder()
        .keys(doc! { field: 1 })
        .options(
            IndexOptions::builder()
                .unique(true)
                .partial_filter_expression(doc! { field: { "$exists": true, "$type": "string"  } })
                .build(),
        )
        .build();

    collection.create_index(index).await?;
    Ok(())
}

pub async fn create_unique_indexes(client: &Client) -> Result<(), MongoError> {
    let collection = client.database(DB_NAME).collection::<User>(USER_COL_NAME);

    create_partial_unique_index(&collection, "email").await?;
    create_partial_unique_index(&collection, "username").await?;
    create_partial_unique_index(&collection, "nim").await?;
    create_partial_unique_index(&collection, "nidn").await?;

    Ok(())
}
