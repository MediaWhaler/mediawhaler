mod config;

use axum::{routing::get, Router, Server};
use std::{fs::File, net::SocketAddr};
use tracing::info;
use tracing_subscriber::{prelude::*, Registry};

#[tokio::main]
async fn main() {
    let stderr_log = tracing_subscriber::fmt::layer()
        .with_ansi(true)
        .with_writer(std::io::stderr);
    let file = File::create("./mediawhaler.log").expect("unable to create log file");
    let file_log = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_writer(file);
    let subscriber = Registry::default().with(stderr_log).with(file_log);

    tracing::subscriber::set_global_default(subscriber)
        .expect("Unable to setup global log subscriber");

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("start listening on {addr}");

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
