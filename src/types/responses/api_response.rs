use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn new(status: u16, message: String, data: Option<T>) -> Self {
        Self {
            status,
            message,
            data,
        }
    }
}

impl<T> Display for ApiResponse<T>
where
    T: Serialize,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        let data_string = match &self.data {
            Some(data) => {
                serde_json::to_string(data).unwrap_or_else(|_| "Serialization error".to_string())
            }
            None => "None".to_string(),
        };

        write!(
            f,
            "Status: {}, Message: {}, Data: {}",
            self.status, self.message, data_string
        )
    }
}
