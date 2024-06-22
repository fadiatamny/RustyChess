use maplit::hashmap;
use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
};
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};
use tracing::{debug, info};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Status {
    NotFound,
    Ok,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::NotFound => write!(f, "404 Not Found"),
            Status::Ok => write!(f, "200 OK"),
        }
    }
}

pub struct Response {
    pub status: Status,
    pub headers: Option<HashMap<String, String>>,
    pub data: Box<dyn AsyncRead + Unpin + Send>,
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

    pub async fn send(mut self, mut stream: impl AsyncWrite + Unpin) -> anyhow::Result<()> {
        let mut buffer = Vec::new();
        let mut reader = self.data.as_mut();
        tokio::io::copy(&mut reader, &mut buffer).await.unwrap();
        let data_length = buffer.len();

        let mut headers = self.generate_headers();

        headers
            .entry("Content-Length".to_string())
            .or_insert_with(|| data_length.to_string());

        let stringified_headers = Self::stringify_headers(headers);
        let bytes =
            format!("HTTP/1.1 {}\r\n{stringified_headers}\r\n\r\n", self.status).into_bytes();

        stream.write_all(&bytes).await?;
        tokio::io::copy(&mut self.data, &mut stream).await?;
        Ok(())
    }
}
