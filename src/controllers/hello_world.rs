use maplit::hashmap;

use crate::models::api_entities::{APIData, APIJsonResponse, APITextResponse};
use serde_json::Value::String;

pub fn get() -> APIData {
    let payload = "Hello, world!".to_string();
    let res = APITextResponse::new(payload);
    APIData::Text(res)
}

pub fn post() -> APIData {
    let payload = hashmap! {
        "message".to_string() => String("Hello, world!".to_string())
    };
    let res = APIJsonResponse::new(payload);

    APIData::JSON(res)
}
