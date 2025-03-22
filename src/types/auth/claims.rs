use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct Claims {
    pub _id: String,
    pub role: String,
    pub email: Option<String>,
    pub exp: usize,
}
