mod persistent_post_repository;
use axum::{
    extract::State,
    http,
    response::{IntoResponse, Response, Result},
    Json,
};
pub use persistent_post_repository::PersistentPostRepository;
use sqlx::PgPool;

use crate::{
    application::posts::{create_post, list_posts},
    domain::{error::ApplicationLayerError, posts::CreatePostDTO},
};

#[axum::debug_handler]
pub async fn create_post_action(
    State(pool): State<PgPool>,
    Json(payload): Json<CreatePostDTO>,
) -> Result<Response, Response> {
    let mut post_repository = PersistentPostRepository::new(&pool);

    create_post(&mut post_repository, payload)
        .await
        .map_err(|err| match err {
            ApplicationLayerError::PersistenceError(_) => (
                http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            )
                .into_response(),
            ApplicationLayerError::ValidationError(violations) => {
                (http::StatusCode::BAD_REQUEST, Json(violations)).into_response()
            }
        })
        .map(|post| (http::StatusCode::CREATED, Json(post)).into_response())
}

#[axum::debug_handler]
pub async fn list_posts_action(
    State(pool): State<PgPool>, /* TODO: handle take&skip query params */
) -> Result<Response, Response> {
    let mut post_repository = PersistentPostRepository::new(&pool);

    list_posts(&mut post_repository)
        .await
        .map(|posts| (http::StatusCode::OK, Json(posts)).into_response())
        .map_err(|err| match err {
            ApplicationLayerError::PersistenceError(_) => (
                http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            )
                .into_response(),
            ApplicationLayerError::ValidationError(violations) => {
                (http::StatusCode::BAD_REQUEST, Json(violations)).into_response()
            }
        })
}
