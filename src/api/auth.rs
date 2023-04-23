use crate::{ApiError, ApiResult};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use axum::routing::post;

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> ApiResult<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO: Implement real db/auth logic.
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(ApiError::LoginFail);
    }

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
