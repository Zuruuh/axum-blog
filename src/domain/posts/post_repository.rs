use async_trait::async_trait;

use super::{Post, ValidatedCreatePostDTO};
use crate::domain::error::ApplicationLayerError;

#[async_trait]
pub trait PostRepository {
    async fn persist(
        &mut self,
        create_post_dto: ValidatedCreatePostDTO,
    ) -> Result<Post, ApplicationLayerError>;
    async fn exists_with_id(&self, id: &uuid::Uuid) -> Result<bool, ApplicationLayerError>;
    async fn exists_with_title(&self, title: &String) -> Result<bool, ApplicationLayerError>;
    async fn list_posts(&self, skip: &i32, take: &i32) -> Result<Vec<Post>, ApplicationLayerError>;
    async fn find_post(&self, id: &uuid::Uuid) -> Result<Option<Post>, ApplicationLayerError>;
    async fn delete_post(&mut self, id: &uuid::Uuid) -> Result<bool, ApplicationLayerError>;
}
