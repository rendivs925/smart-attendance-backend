use email_address::EmailAddress;
use phonenumber::{country, parse};
use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(custom(function = "validate_identifier"))]
    pub identifier: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

fn validate_identifier(identifier: &str) -> Result<(), ValidationError> {
    if EmailAddress::is_valid(identifier) || is_valid_phone_number(identifier) {
        Ok(())
    } else {
        Err(ValidationError::new(
            "Invalid identifier: must be a valid email or phone number",
        ))
    }
}

fn is_valid_phone_number(identifier: &str) -> bool {
    parse(Some(country::ID), identifier)
        .map(|num| num.is_valid())
        .unwrap_or(false)
}
