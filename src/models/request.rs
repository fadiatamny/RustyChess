use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Options
}

impl TryFrom<&str> for Method {
    type Error = String;

    fn try_from(method: &str) -> Result<Self, Self::Error> {
        match method {
            "GET" => Ok(Method::Get),
            _ => Err(format!("unsupported method: {}", method))
        }
    }
}