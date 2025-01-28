use crate::types::api_response::ApiResponse;
use actix_web::HttpResponse;

pub fn create_response<T>(status: u16, message: &str, data: Option<T>) -> HttpResponse
where
    T: serde::Serialize,
{
    HttpResponse::build(actix_web::http::StatusCode::from_u16(status).unwrap())
        .json(ApiResponse::new(status, message.to_string(), data))
}
