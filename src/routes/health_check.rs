use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub const ENDPOINT: &str = "/health";

pub async fn handler() -> Response {
    let response = r#"{ "status": "UP" }"#;

    (StatusCode::OK, response).into_response()
}
