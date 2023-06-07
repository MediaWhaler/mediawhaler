use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sqlx;

pub type Result<T> = core::result::Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    LoginFail,
    SqlxError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        println!("error from sqlx {}", err);
        ApiError::SqlxError
    }
}
