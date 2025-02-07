use crate::{models::attendance_model::Attendance, utils::api_utils::create_response};
use actix_web::{web, HttpResponse, Responder};

pub async fn create_attendance_handler(attendance: web::Json<Attendance>) -> impl Responder {
    match Attendance::create_attendance(attendance.into_inner()).await {
        Ok(new_attendance) => {
            create_response(201, "Attendance created successfully", Some(new_attendance))
        }
        Err(err) => {
            eprintln!("Error creating attendance: {}", err);
            create_response(500, "Failed to create attendance", Some(err))
        }
    }
}

pub async fn get_all_attendances_handler() -> HttpResponse {
    match Attendance::get_all_attendances().await {
        Ok(attendances) => {
            create_response(200, "Attendances fetched successfully", Some(attendances))
        }
        Err(err) => {
            eprintln!("Error fetching attendances: {}", err);
            create_response(500, "Error fetching attendances", Some(err))
        }
    }
}

pub async fn get_attendance_handler(attendance_id: web::Path<String>) -> HttpResponse {
    match Attendance::find_attendance_by_id(&attendance_id).await {
        Ok(Some(attendance)) => {
            create_response(200, "Attendance found successfully", Some(attendance))
        }
        Ok(None) => create_response::<String>(404, "Attendance not found", None),
        Err(err) => create_response(500, "Error fetching attendance", Some(err)),
    }
}

pub async fn update_attendance_handler(
    attendance_id: web::Path<String>,
    attendance: web::Json<Attendance>,
) -> HttpResponse {
    match Attendance::update_attendance(&attendance_id, &attendance.into_inner()).await {
        Ok(updated_attendance) => create_response(
            200,
            "Attendance updated successfully",
            Some(updated_attendance),
        ),
        Err(err) => {
            eprintln!("Error updating attendance: {}", err);
            create_response(500, "Error updating attendance", Some(err))
        }
    }
}

pub async fn delete_attendance_handler(attendance_id: web::Path<String>) -> HttpResponse {
    match Attendance::delete_attendance(&attendance_id).await {
        Ok(_) => create_response::<String>(204, "Attendance deleted successfully", None),
        Err(err) => {
            eprintln!("Error deleting attendance: {}", err);
            create_response(500, "Error deleting attendance", Some(err))
        }
    }
}
