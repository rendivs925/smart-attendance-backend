use crate::{
    config::database::{get_collection, ORGANIZATIONS_COL_NAME},
    models::organization_model::Organization,
};
use futures_util::stream::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId, to_document};
use mongodb::{error::Result, Client, Collection};

pub struct OrganizationRepository {
    collection: Collection<Organization>,
}

impl OrganizationRepository {
    pub async fn new(client: &Client) -> Result<Self> {
        let collection = get_collection(client, ORGANIZATIONS_COL_NAME).await?;
        Ok(Self { collection })
    }

    pub async fn create_organization(&self, organization: &Organization) -> Result<Organization> {
        let insert_result = self.collection.insert_one(organization).await?;
        Ok(Organization {
            _id: Some(insert_result.inserted_id.as_object_id().unwrap()),
            ..organization.clone()
        })
    }

    pub async fn find_organization_by_id(&self, org_id: &str) -> Result<Option<Organization>> {
        let object_id = ObjectId::parse_str(org_id).unwrap();
        self.collection.find_one(doc! { "_id": object_id }).await
    }

    pub async fn get_all_organizations(&self) -> Result<Vec<Organization>> {
        let cursor = self.collection.find(doc! {}).await?;
        let organizations: Vec<Organization> = cursor.try_collect().await?;
        Ok(organizations)
    }

    pub async fn update_organization(
        &self,
        org_id: &str,
        organization: &Organization,
    ) -> Result<Organization> {
        let object_id = ObjectId::parse_str(org_id).unwrap();
        let update_doc = to_document(organization)?;

        self.collection
            .update_one(doc! { "_id": object_id }, doc! { "$set": update_doc })
            .await?;

        Ok(organization.clone())
    }

    pub async fn delete_organization(&self, org_id: &str) -> Result<()> {
        let object_id = ObjectId::parse_str(org_id).unwrap();
        self.collection
            .delete_one(doc! { "_id": object_id })
            .await?;
        Ok(())
    }
}
