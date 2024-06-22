use crate::models::api_entities::APIData;
use maplit::hashmap;
use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
};
use tokio::io::{AsyncWrite, AsyncWriteExt};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Status {
    NotFound,
    Ok,
    Error,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::NotFound => write!(f, "404 Not Found"),
            Status::Ok => write!(f, "200 OK"),
            Status::Error => write!(f, "500 Internal Server Error"),
        }
    }
}

pub struct Response {
    pub status: Status,
    pub headers: Option<HashMap<String, String>>,
    pub data: APIData,
}

impl Response {
    fn generate_headers(&self) -> HashMap<String, String> {
        return self.headers.clone().unwrap_or_else(|| {
            hashmap! {
                "Content-Type".to_string() => "text/plain".to_string(),
                "Content-Length".to_string() => "0".to_string()
            }
        });
    }
    fn stringify_headers(headers: HashMap<String, String>) -> String {
        let response = headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\r\n");

        return response;
    }

    pub async fn send(&self, mut stream: impl AsyncWrite + Unpin) -> anyhow::Result<()> {
        let content: String;
        let content_type: String;
        let content_length: usize;

        match &self.data {
            APIData::Text(data) => {
                content = data.to_string();
                content_type = data.content_type.clone();
                content_length = content.len();
            }
            APIData::JSON(data) => {
                content = data.to_string();
                content_type = data.content_type.clone();
                content_length = content.len();
            }
        };

        let mut headers = self.generate_headers();

        if let Some(ctype) = headers.get_mut("Content-Type") {
            *ctype = content_type;
        } else {
            headers.insert("Content-Type".to_string(), content_type);
        }

        if let Some(clen) = headers.get_mut("Content-Length") {
            *clen = content_length.to_string();
        } else {
            headers.insert("Content-Length".to_string(), content_length.to_string());
        }

        let stringified_headers = Self::stringify_headers(headers);
        let response = format!(
            "HTTP/1.1 {}\r\n{stringified_headers}\r\n\r\n{content}",
            self.status
        );

        let bytes = response.into_bytes();
        stream.write_all(&bytes).await?;
        stream.flush().await?;

        Ok(())
    }
}
