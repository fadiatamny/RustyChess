use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use serde::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Serialize, Deserialize, Debug)]
pub struct APITextResponse {
    data: String,
    pub content_type: String,
}

impl APITextResponse {
    pub fn new(data: String) -> Self {
        APITextResponse {
            data,
            content_type: "text/plain".to_string(),
        }
    }
}

impl Display for APITextResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

type JSONValue = serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct APIJsonResponse {
    data: HashMap<String, JSONValue>,
    pub content_type: String,
}

impl APIJsonResponse {
    pub fn new(data: HashMap<String, JSONValue>) -> Self {
        APIJsonResponse {
            data,
            content_type: "application/json".to_string(),
        }
    }
}

impl Display for APIJsonResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Ok(json) = to_string(&self.data) {
            write!(f, "{}", json)
        } else {
            write!(f, "{{}}")
        }
    }
}

pub enum APIData {
    Text(APITextResponse),
    JSON(APIJsonResponse),
}
