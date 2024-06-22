use std::error;
use std::fmt;

#[derive(Debug)]
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.code, self.message)
    }
}

impl error::Error for APIError {}


impl APIError {
    pub fn not_found(message: impl Into<String>) -> Self {
        let str = message.into();
        APIError::new(404, format!("Not Found - {}", str))
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        let str = message.into();
        APIError::new(404, format!("Unauthorized - {}", str))
    }

    pub fn internal_server_error(message: impl Into<String>) -> Self {
        let str = message.into();
        APIError::new(500, format!("Internal Server Error - {}", str))
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        let str = message.into();
        APIError::new(400, format!("Bad Request - {}", str))
    }
}