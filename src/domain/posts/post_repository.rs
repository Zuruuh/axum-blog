use async_trait::async_trait;

use super::{CreatePostDTO, Post};
use crate::domain::persistence::PersistenceError;

#[async_trait]
pub trait PostRepository {
    async fn persist(&mut self, create_post_dto: CreatePostDTO) -> Result<Post, PersistenceError>;
}
