use std::net::SocketAddr;

use anyhow::{anyhow, Context, Result};
use axum::{
    extract::Host,
    handler::HandlerWithoutStateExt,
    http::{StatusCode, Uri},
    response::Redirect,
    BoxError, Router, Server,
};
use axum_server::tls_rustls::RustlsConfig;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("failed to bind to address {0}")]
    AddressBindingError(String),
}

pub async fn start_http(addr: &SocketAddr, app: Router) -> Result<()> {
    let server = Server::try_bind(addr)
        .with_context(|| ServerError::AddressBindingError(addr.to_string()))?
        .serve(app.into_make_service());

    tracing::info!("start listening on {addr}");
    server.await.map_err(|e| anyhow!(e))
}

pub async fn start_https(addr: &SocketAddr, app: Router, tls: RustlsConfig) -> Result<()> {
    let server = axum_server::bind_rustls(*addr, tls).serve(app.into_make_service());
    tracing::info!("start listening for https connection on {addr}");
    server.await.map_err(|e| anyhow!(e))
}

pub async fn start_http_to_https_redirect(http: u16, https: u16) -> Result<()> {
    fn make_https(host: String, uri: Uri, http: u16, https: u16) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let https_host = host.replace(&http.to_string(), &https.to_string());

        parts.authority = Some(https_host.parse()?);
        Ok(Uri::from_parts(parts)?)
    }

    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(host, uri, http, https) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(error) => {
                tracing::warn!(%error, "failed to convert URI to HTTPS");
                Err(StatusCode::BAD_REQUEST)
            }
        }
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], http));

    tracing::info!("start listening for http connection on {addr}");
    let server = axum::Server::try_bind(&addr)
        .with_context(|| ServerError::AddressBindingError(addr.to_string()))?
        .serve(redirect.into_make_service());

    server.await.map_err(|e| anyhow!(e))
}
