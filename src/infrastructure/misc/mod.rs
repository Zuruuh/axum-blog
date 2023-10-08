use axum::http;

pub async fn health_route() -> http::StatusCode { http::StatusCode::NO_CONTENT }
