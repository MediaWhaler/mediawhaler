use anyhow::{anyhow, Context};
use axum::{routing::get, Router};
use axum_server::tls_rustls::RustlsConfig;
use mediawhaler::config;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let conf = config::Config::new().context("unable to get config")?;
    dbg!(&conf);

    // Keep a reference on the non blocking writters guards
    let _guard = mediawhaler::logs::setup(&conf.logs)?;

    let app = Router::new().route("/", get(handler));

    if let Err(error) = match &conf.network.https {
        Some(https) => {
            tokio::spawn(mediawhaler::server::start_http_to_https_redirect(
                conf.network.http.port,
                https.port,
            ));
            dbg!(https);
            let tls_config = RustlsConfig::from_pem_file(https.cert.clone(), https.key.clone())
                .await
                .map_err(|e| anyhow!("unable to get cert files: {e}"))?;
            let addr = SocketAddr::from(([127, 0, 0, 1], https.port));
            mediawhaler::server::start_https(&addr, app, tls_config).await
        }
        None => {
            let addr = SocketAddr::from(([127, 0, 0, 1], conf.network.http.port));
            mediawhaler::server::start_http(&addr, app).await
        }
    } {
        tracing::error!("{error:?}")
    }

    Ok(())
}

async fn handler() -> &'static str {
    tracing::debug!("this is a debug message from an async block");
    "Hello, World!"
}
