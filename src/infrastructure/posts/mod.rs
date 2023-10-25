mod persistent_post_repository;
use axum::{
    extract::{Path, Query, State},
    http,
    response::{IntoResponse, Response, Result},
    Json,
};
pub use persistent_post_repository::PersistentPostRepository;
use sqlx::PgPool;

use crate::{
    application::posts::{create_post, delete_post, find_post, list_posts, update_post},
    domain::{
        error::ApplicationLayerError,
        pagination::PaginationOptions,
        posts::{CreatePostDTO, UpdatePostDTO},
    },
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
    State(pool): State<PgPool>,
    Query(pagination_options): Query<PaginationOptions>,
) -> Result<Response, Response> {
    let post_repository = PersistentPostRepository::new(&pool);

    list_posts(&post_repository, &pagination_options)
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

#[axum::debug_handler]
pub async fn find_post_action(
    State(pool): State<PgPool>,
    Path(post_id): Path<uuid::Uuid>,
) -> Result<Response, Response> {
    let post_repository = PersistentPostRepository::new(&pool);

    find_post(&post_repository, &post_id)
        .await
        .map(|post| match post {
            None => (http::StatusCode::NOT_FOUND).into_response(),
            Some(post) => (http::StatusCode::FOUND, Json(post)).into_response(),
        })
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

#[axum::debug_handler]
pub async fn delete_post_action(
    State(pool): State<PgPool>,
    Path(post_id): Path<uuid::Uuid>,
) -> Result<Response, Response> {
    let mut post_repository = PersistentPostRepository::new(&pool);

    delete_post(&mut post_repository, &post_id)
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
        .map(|deleted| match deleted {
            true => (http::StatusCode::NO_CONTENT).into_response(),
            false => (http::StatusCode::NOT_FOUND).into_response(),
        })
}

#[axum::debug_handler]
pub async fn update_post_action(
    State(pool): State<PgPool>,
    Path(post_id): Path<uuid::Uuid>,
    Json(update_post_dto): Json<UpdatePostDTO>,
) -> Result<Response, Response> {
    let mut post_repository = PersistentPostRepository::new(&pool);

    update_post(&mut post_repository, &post_id, update_post_dto)
        .await
        .map(|post| (http::StatusCode::OK, Json(post)).into_response())
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
