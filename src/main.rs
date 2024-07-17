mod models;
mod routers;
mod controllers;
mod game;

use models::{request::Request, router::Router};

use tokio::{io::BufStream, net::TcpListener, signal};
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

    let router = routers::get_routers();

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

    let req = Request::parse(&mut buffer).await?;
    let res = router.handle(&req);

    if let Err(e) = res.send(&mut buffer).await {
        error!("Error writing response: {}", e);
    }

    Ok(())
}
