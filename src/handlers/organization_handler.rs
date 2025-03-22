use crate::{
    models::organization_model::Organization, services::organization_service::OrganizationService,
    utils::api_utils::create_response,
};
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

pub async fn create_organization_handler(
    organization_service: web::Data<Arc<OrganizationService>>,
    organization: web::Json<Organization>,
) -> impl Responder {
    match organization_service
        .create_organization(organization.into_inner())
        .await
    {
        Ok(new_org) => create_response(201, "Organization created successfully", Some(new_org)),
        Err(err) => create_response(500, "Failed to create organization", Some(err.to_string())),
    }
}

pub async fn get_organization_handler(
    organization_service: web::Data<Arc<OrganizationService>>,
    org_id: web::Path<String>,
) -> HttpResponse {
    match organization_service.get_organization_by_id(&org_id).await {
        Ok(Some(organization)) => {
            create_response(200, "Organization found successfully", Some(organization))
        }
        Ok(None) => create_response::<String>(404, "Organization not found", None),
        Err(err) => create_response(500, "Error fetching organization", Some(err.to_string())),
    }
}

pub async fn get_all_organizations_handler(
    organization_service: web::Data<Arc<OrganizationService>>,
) -> HttpResponse {
    match organization_service.get_all_organizations().await {
        Ok(orgs) => create_response(200, "Organizations fetched successfully", Some(orgs)),
        Err(err) => create_response(500, "Error fetching organizations", Some(err.to_string())),
    }
}

pub async fn update_organization_handler(
    organization_service: web::Data<Arc<OrganizationService>>,
    org_id: web::Path<String>,
    organization: web::Json<Organization>,
) -> HttpResponse {
    match organization_service
        .update_organization(&org_id, organization.into_inner())
        .await
    {
        Ok(updated_org) => {
            create_response(200, "Organization updated successfully", Some(updated_org))
        }
        Err(err) => create_response(500, "Error updating organization", Some(err.to_string())),
    }
}

pub async fn delete_organization_handler(
    organization_service: web::Data<Arc<OrganizationService>>,
    org_id: web::Path<String>,
) -> HttpResponse {
    match organization_service.delete_organization(&org_id).await {
        Ok(_) => create_response::<String>(204, "Organization deleted successfully", None),
        Err(err) => create_response(500, "Error deleting organization", Some(err.to_string())),
    }
}
