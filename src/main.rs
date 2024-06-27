use api_entities::{APIData, APIJsonResponse, APITextResponse};
use maplit::hashmap;
use response::{Response, Status};
use router::Router;
use tokio::{io::BufStream, net::TcpListener, signal};
mod models;
use models::*;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

static PORT: u16 = 3000;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let port = std::env::var("PORT").unwrap_or_else(|_| PORT.to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    let cancel_token = CancellationToken::new();

    info!("Listening on port: {port}");
    info!("{}", listener.local_addr().unwrap());
    info!("Press Ctrl+C to stop the server...");

    let mut child_router = Router::new();
    child_router.get("/world", |req| Response {
        status: Status::Ok,
        headers: None,
        data: APIData::Text(APITextResponse::new("World".to_string())),
    });

    let mut router = Router::new();

    router.get("/hello", |req| Response {
        status: Status::Ok,
        headers: None,
        data: APIData::Text(APITextResponse::new("Hello".to_string())),
    });
    router.post("/test", |req| Response {
        status: Status::Ok,
        headers: None,
        data: models::api_entities::APIData::JSON(APIJsonResponse::new(
            hashmap! {
                "test".to_string() => serde_json::Value::Bool(true)
            },
        )),
    });

    router.use_router("/child", child_router);

    tokio::spawn({
        let cancel_token = cancel_token.clone();
        async move {
            if let Ok(()) = signal::ctrl_c().await {
                info!("received Ctrl-C, shutting down");
                cancel_token.cancel();
            }
        }
    });

    let mut tasks: Vec<tokio::task::JoinHandle<()>> = Vec::new();

    loop {
        let cancel_token = cancel_token.clone();

        tokio::select! {
            Ok((stream, addr)) = listener.accept() => {
                let router_clone = router.clone();
                let client_task = tokio::spawn(async move {
                    info!("Accepted connection from: {addr}");
                    if let  Err(e) = handle_connection(stream, router_clone).await {
                        error!("Error handling connection: {e}");
                    }
                });
                tasks.push(client_task);
            },
            _ = cancel_token.cancelled() => {
                info!("stop listening");
                break;
            }
        }
    }

    futures::future::join_all(tasks).await;
    Ok(())
}

async fn handle_connection(stream: tokio::net::TcpStream, router: Router) -> anyhow::Result<()> {
    let mut buffer = BufStream::new(stream);

    let req = request::Request::parse(&mut buffer).await?;
    let res = router.handle(&req);

    if let Err(e) = res.send(&mut buffer).await {
        error!("Error writing response: {}", e);
    }

    Ok(())
}
