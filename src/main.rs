use anyhow::{anyhow, Result, Context};
use axum::{routing::get, Router};
use axum_server::tls_rustls::RustlsConfig;
use mediawhaler::{config, directories::Dirs};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    let dirs = Dirs::new().ok_or(anyhow!("unable to create dirs"))?;
    let conf = config::Config::new(&dirs).context("unable to get config")?;

    // Keep a reference on the non blocking writters guards
    let _guard = mediawhaler::logs::setup(&conf.logs)?;

    let app = Router::new().route("/", get(handler));

    if let Err(error) = match &conf.network.https {
        Some(https) => {
            tokio::spawn(mediawhaler::server::start_http_to_https_redirect(
                conf.network.http.port,
                https.port,
            ));
            let tls_config = RustlsConfig::from_pem_file(https.cert.clone(), https.key.clone())
                .await
                .map_err(|e| anyhow!("unable to get cert files: {e}"))?;
            let addr = SocketAddr::from(([127, 0, 0, 1], https.port));
            mediawhaler::server::start_https(&addr, app, tls_config).await.with_context(|| format!("unable to start listening for https connexion on {addr}"))
        }
        None => {
            let addr = SocketAddr::from(([127, 0, 0, 1], conf.network.http.port));
            mediawhaler::server::start_http(&addr, app).await.with_context(|| format!("unable to start listening for http connexion on {addr}"))
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
