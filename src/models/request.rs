use maplit::hashmap;
use tokio::io::{AsyncBufRead, AsyncBufReadExt};

use crate::models::errors::APIErrors;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Options,
}

impl TryFrom<&str> for Method {
    type Error = APIErrors;

    fn try_from(method: &str) -> Result<Self, Self::Error> {
        match method {
            "GET" => Ok(Method::Get),
            _ => Err(APIErrors::internal_server_error("Method not allowed")),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
}

impl Request {
    pub async fn parse(mut stream: impl AsyncBufRead + Unpin) -> anyhow::Result<Self> {
        let mut line = String::new();

        let mut req = Request {
            method: Method::Get,
            path: String::new(),
            headers: hashmap! {},
        };

        let mut stage: u8 = 0;

        loop {
            stream.read_line(&mut line).await?;

            if line.is_empty() || line == "\r\n" || line == "\n" {
                break;
            }

            match stage {
                0 => {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    req.method = Method::try_from(parts[0])?;
                    req.path = parts[1].to_string();
                }
                _ => {
                    let parts: Vec<&str> = line.split(":").collect();
                    req.headers
                        .insert(parts[0].trim().to_string(), parts[1].trim().to_string());
                }
            }

            stage += 1;

            /**
             * todo:
             * 1. Parse the request line
             * 2. Parse the headers
             * 3. Parse the body if valid json or form data. if  post.
             */

            line.clear();
        }

        Ok(req)
    }
}
