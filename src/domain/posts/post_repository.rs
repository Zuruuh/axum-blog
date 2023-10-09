use async_trait::async_trait;

use super::{Post, ValidatedCreatePostDTO};
use crate::domain::persistence::PersistenceError;

#[async_trait]
pub trait PostRepository {
    async fn persist(
        &mut self,
        create_post_dto: ValidatedCreatePostDTO,
    ) -> Result<Post, PersistenceError>;
    async fn exists_with_id(&self, id: &uuid::Uuid) -> Result<bool, PersistenceError>;
    async fn exists_with_title(&self, title: &String) -> Result<bool, PersistenceError>;
}
