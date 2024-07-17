use thiserror::Error;
use std::fmt;

#[derive(Error, Debug)]
pub struct APIError {
    code: u16,
    message: String,
}

impl APIError {
    pub fn new(code: u16, message: impl Into<String>) -> Self {
        APIError {
            code,
            message: message.into(),
        }
    }
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.code, self.message)
    }
}

#[derive(Error, Debug)]
pub enum APIErrors {
    #[error("Not Found: {0}")]
    NotFound(APIError),
    #[error("Bad Request: {0}")]
    BadRequest(APIError),
    #[error("Internal Server Error: {0}")]
    InternalServerError(APIError)
}


impl APIErrors {
    pub fn internal_server_error(message: impl Into<String>) -> Self {
        APIErrors::InternalServerError(APIError::new(500, message))
    }
}