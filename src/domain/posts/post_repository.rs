use async_trait::async_trait;

use super::{Post, ValidatedCreatePostDTO};
use crate::domain::error::ApplicationError;

#[async_trait]
pub trait PostRepository {
    async fn persist(
        &mut self,
        create_post_dto: ValidatedCreatePostDTO,
    ) -> Result<Post, ApplicationError>;
    async fn exists_with_id(&self, id: &uuid::Uuid) -> Result<bool, ApplicationError>;
    async fn exists_with_title(&self, title: &String) -> Result<bool, ApplicationError>;
    async fn list_posts(&self, skip: &i32, take: &i32) -> Result<Vec<Post>, ApplicationError>;
    async fn find_post(&self, id: &uuid::Uuid) -> Result<Option<Post>, ApplicationError>;
    async fn delete_post(&mut self, id: &uuid::Uuid) -> Result<bool, ApplicationError>;
    async fn update_post(&mut self, post: &Post) -> Result<bool, ApplicationError>;
}
