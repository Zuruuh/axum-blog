use axum::response::Response;

use crate::domain::error::ApplicationError;

#[axum::debug_handler]
pub async fn register() -> Result<Response, ApplicationError> {
    todo!()
}
