use crate::{
    models::attendance_model::Attendance, services::attendance_service::AttendanceService,
    utils::api_utils::create_response,
};
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

pub async fn create_attendance_handler(
    attendance_service: web::Data<Arc<AttendanceService>>,
    attendance: web::Json<Attendance>,
) -> impl Responder {
    match attendance_service
        .create_attendance(attendance.into_inner())
        .await
    {
        Ok(new_attendance) => {
            create_response(201, "Attendance created successfully", Some(new_attendance))
        }
        Err(err) => create_response(500, "Failed to create attendance", Some(err.to_string())),
    }
}

pub async fn get_all_attendances_handler(
    attendance_service: web::Data<Arc<AttendanceService>>,
) -> HttpResponse {
    match attendance_service.get_all_attendances().await {
        Ok(attendances) => {
            create_response(200, "Attendances fetched successfully", Some(attendances))
        }
        Err(err) => create_response(500, "Error fetching attendances", Some(err.to_string())),
    }
}

pub async fn get_attendance_handler(
    attendance_service: web::Data<Arc<AttendanceService>>,
    attendance_id: web::Path<String>,
) -> HttpResponse {
    match attendance_service
        .get_attendance_by_id(&attendance_id)
        .await
    {
        Ok(Some(attendance)) => {
            create_response(200, "Attendance found successfully", Some(attendance))
        }
        Ok(None) => create_response::<String>(404, "Attendance not found", None),
        Err(err) => create_response(500, "Error fetching attendance", Some(err.to_string())),
    }
}

pub async fn update_attendance_handler(
    attendance_service: web::Data<Arc<AttendanceService>>,
    attendance_id: web::Path<String>,
    attendance: web::Json<Attendance>,
) -> HttpResponse {
    match attendance_service
        .update_attendance(&attendance_id, attendance.into_inner())
        .await
    {
        Ok(updated_attendance) => create_response(
            200,
            "Attendance updated successfully",
            Some(updated_attendance),
        ),
        Err(err) => create_response(500, "Error updating attendance", Some(err.to_string())),
    }
}

pub async fn delete_attendance_handler(
    attendance_service: web::Data<Arc<AttendanceService>>,
    attendance_id: web::Path<String>,
) -> HttpResponse {
    match attendance_service.delete_attendance(&attendance_id).await {
        Ok(_) => create_response::<String>(204, "Attendance deleted successfully", None),
        Err(err) => create_response(500, "Error deleting attendance", Some(err.to_string())),
    }
}
