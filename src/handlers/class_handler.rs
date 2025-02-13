use crate::models::class_model::Class;
use crate::services::class_service::ClassService;
use crate::utils::api_utils::create_response;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

pub async fn create_class_handler(
    class_service: web::Data<Arc<ClassService>>,
    class: web::Json<Class>,
) -> impl Responder {
    match class_service.create_class(&class.into_inner()).await {
        Ok(new_class) => create_response(201, "Class created successfully", Some(new_class)),
        Err(err) => create_response(500, "Failed to create class", Some(err.to_string())),
    }
}

pub async fn get_all_classes_handler(class_service: web::Data<Arc<ClassService>>) -> HttpResponse {
    match class_service.get_all_classes().await {
        Ok(classes) => create_response(200, "Classes fetched successfully", Some(classes)),
        Err(err) => create_response(500, "Error fetching classes", Some(err.to_string())),
    }
}

pub async fn get_class_handler(
    class_service: web::Data<Arc<ClassService>>,
    class_id: web::Path<String>,
) -> HttpResponse {
    match class_service.get_class_by_id(&class_id).await {
        Ok(Some(class)) => create_response(200, "Class found successfully", Some(class)),
        Ok(None) => create_response::<String>(404, "Class not found", None),
        Err(err) => create_response(500, "Error fetching class", Some(err.to_string())),
    }
}

pub async fn update_class_handler(
    class_service: web::Data<Arc<ClassService>>,
    class_id: web::Path<String>,
    class: web::Json<Class>,
) -> HttpResponse {
    match class_service
        .update_class(&class_id, &class.into_inner())
        .await
    {
        Ok(updated_class) => {
            create_response(200, "Class updated successfully", Some(updated_class))
        }
        Err(err) => create_response(500, "Error updating class", Some(err.to_string())),
    }
}

pub async fn delete_class_handler(
    class_service: web::Data<Arc<ClassService>>,
    class_id: web::Path<String>,
) -> HttpResponse {
    match class_service.delete_class(&class_id).await {
        Ok(_) => create_response::<String>(204, "Class deleted successfully", None),
        Err(err) => create_response(500, "Error deleting class", Some(err.to_string())),
    }
}
