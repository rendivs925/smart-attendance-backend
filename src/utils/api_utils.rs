use actix_web::{http::StatusCode, HttpResponse};
use serde::Serialize;

use crate::types::responses::api_response::ApiResponse;

pub fn create_response<T>(status: u16, message: &str, data: Option<T>) -> HttpResponse
where
    T: Serialize,
{
    let status_code = StatusCode::from_u16(status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    HttpResponse::build(status_code).json(ApiResponse::new(status, message.to_string(), data))
}
