use crate::models::user_model::User;
use mongodb::IndexModel;
use mongodb::{
    bson::doc,
    options::{ClientOptions, IndexOptions},
    Client,
};
use std::{env, error::Error};

pub const DB_NAME: &str = "smart-attendance";

pub async fn connect_to_database() -> Result<Client, Box<dyn Error>> {
    let client_uri =
        env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

    let client_options = ClientOptions::parse(&client_uri).await?;

    let client = Client::with_options(client_options)?;

    Ok(client)
}

async fn create_unique_index(
    collection: &mongodb::Collection<User>,
    field: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let index = IndexModel::builder()
        .keys(doc! { field: 1 })
        .options(IndexOptions::builder().unique(true).build())
        .build();

    collection.create_index(index).await?;
    Ok(())
}

async fn ensure_field_exists_and_create_index(
    collection: &mongodb::Collection<User>,
    field: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let filter = doc! { field: { "$exists": true, "$ne": null } };
    let count = collection.count_documents(filter).await?;

    if count > 0 {
        create_unique_index(collection, field).await?;
    }
    Ok(())
}

pub async fn create_unique_indexes() -> Result<(), Box<dyn std::error::Error>> {
    let client = connect_to_database().await?;
    let collection = client.database(DB_NAME).collection::<User>("users");

    ensure_field_exists_and_create_index(&collection, "email").await?;
    ensure_field_exists_and_create_index(&collection, "username").await?;
    ensure_field_exists_and_create_index(&collection, "nim").await?;
    ensure_field_exists_and_create_index(&collection, "nidn").await?;

    Ok(())
}
