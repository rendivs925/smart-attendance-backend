use crate::models::class_model::Class;
use crate::utils::api_utils::create_response;
use actix_web::{web, HttpResponse, Responder};

pub async fn create_class_handler(class: web::Json<Class>) -> impl Responder {
    match Class::create_class(&class.into_inner()).await {
        Ok(new_class) => create_response(201, "Class created successfully", Some(new_class)),
        Err(err) => {
            eprintln!("Error creating class: {}", err);
            create_response(500, "Failed to create class", Some(err))
        }
    }
}

pub async fn get_all_classes_handler() -> HttpResponse {
    match Class::get_all_classes().await {
        Ok(classes) => create_response(200, "Classes fetched successfully", Some(classes)),
        Err(err) => {
            eprintln!("Error fetching classes: {}", err);
            create_response(500, "Error fetching classes", Some(err))
        }
    }
}

pub async fn get_class_handler(class_id: web::Path<String>) -> HttpResponse {
    match Class::find_class_by_id(&class_id).await {
        Ok(Some(class)) => create_response(200, "Class found successfully", Some(class)),
        Ok(None) => create_response::<String>(404, "Class not found", None),
        Err(err) => create_response(500, "Error fetching class", Some(err)),
    }
}

pub async fn update_class_handler(
    class_id: web::Path<String>,
    class: web::Json<Class>,
) -> HttpResponse {
    match Class::update_class(&class_id, &class.into_inner()).await {
        Ok(updated_class) => {
            create_response(200, "Class updated successfully", Some(updated_class))
        }
        Err(err) => {
            eprintln!("Error updating class: {}", err);
            create_response(500, "Error updating class", Some(err))
        }
    }
}

pub async fn delete_class_handler(class_id: web::Path<String>) -> HttpResponse {
    match Class::delete_class(&class_id).await {
        Ok(_) => create_response::<String>(204, "Class deleted successfully", None),
        Err(err) => {
            eprintln!("Error deleting class: {}", err);
            create_response(500, "Error deleting class", Some(err))
        }
    }
}
