mod persistent_post_repository;
use axum::{extract::State, http, Json};
pub use persistent_post_repository::PersistentPostRepository;
use sqlx::PgPool;

use crate::{
    application::posts::create_post,
    domain::{persistence::PersistenceError, posts::Post},
};

// #[derive(Debug, serde::Deserialize)]
// struct CreatePostDTO {
//     id: Option<uuid::Uuid>,
// }

#[axum::debug_handler]
pub async fn create_post_route(
    State(pool): State<PgPool>,
    Json(payload): Json<CreatePostDTO>,
) -> Result<(http::StatusCode, Json<Post>), (http::StatusCode, String)> {
    let mut post_repository = PersistentPostRepository::new(&pool);

    create_post(&mut post_repository, payload)
        .await
        .map_err(|err| match err {
            PersistenceError::UncheckedError(_) => (
                http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".into(),
            ),
        })
        .map(|post| (http::StatusCode::CREATED, Json(post)))
}
