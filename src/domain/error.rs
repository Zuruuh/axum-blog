use axum::{http::StatusCode, response::IntoResponse, Json};

use super::validation::ConstraintViolation;

#[derive(Debug)]
pub enum ApplicationError {
    PersistenceError(Box<dyn std::error::Error>),
    ValidationError(Vec<ConstraintViolation>),
}

impl IntoResponse for ApplicationError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApplicationError::PersistenceError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            )
                .into_response(),
            ApplicationError::ValidationError(violations) => {
                (StatusCode::BAD_REQUEST, Json(violations)).into_response()
            }
        }
    }
}
