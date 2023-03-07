use anyhow::{anyhow, Context};
use axum::{routing::get, Router, Server};
use std::net::SocketAddr;
use tracing::{error, info, debug};

use mediawhaler::config;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let conf = config::Config::new().context("unable to get config")?;
    dbg!(&conf);

    // Keep a reference on the non blocking writters guards
    let _guard = mediawhaler::logs::setup(&conf.logs)?;

    let app = Router::new().route("/", get(|| async {
        debug!("this is a debug message from an async block");
        "Hello, World!"
    }));

    let addr = SocketAddr::from(([127, 0, 0, 1], conf.http.port));

    if let Err(error) = start(&addr, app).await {
        error!("server error: {:#}", error);
    }

    Ok(())
}

async fn start(addr: &SocketAddr, app: Router) -> Result<(), anyhow::Error> {
    let server = Server::try_bind(&addr)
        .context("unable to bind address")?
        .serve(app.into_make_service());

    info!("start listening on {addr}");
    server.await.map_err(|e| anyhow!(e))
}
