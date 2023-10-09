use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::domain::{
    error::ApplicationLayerError,
    posts::{Post, PostRepository, ValidatedCreatePostDTO},
};

#[derive(Clone)]
pub struct PersistentPostRepository<'a> {
    pool: &'a Pool<Postgres>,
}

impl<'a> PersistentPostRepository<'a> {
    pub fn new(pool: &'a Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl<'a> PostRepository for PersistentPostRepository<'a> {
    async fn persist(
        &mut self,
        create_post_dto: ValidatedCreatePostDTO,
    ) -> Result<Post, ApplicationLayerError> {
        let post = Post::new(create_post_dto);

        sqlx::query!(
            r#"
            INSERT INTO app_posts (id, title, content, created_at, last_update)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            post.id,
            post.title,
            post.content,
            post.created_at,
            post.last_update,
        )
        .execute(self.pool)
        .await
        .map_err(|err| ApplicationLayerError::PersistenceError(Box::new(err)))?;

        Ok(post)
    }

    async fn exists_with_id(&self, id: &uuid::Uuid) -> Result<bool, ApplicationLayerError> {
        sqlx::query!(
            "SELECT COUNT(*) FROM app_posts AS post WHERE post.id = $1::uuid",
            id
        )
        .fetch_one(self.pool)
        .await
        .map_err(|err| ApplicationLayerError::PersistenceError(Box::new(err)))
        .map(|record| record.count.unwrap_or_default() > 0)
    }

    async fn exists_with_title(&self, title: &String) -> Result<bool, ApplicationLayerError> {
        sqlx::query!(
            "SELECT COUNT(*) FROM app_posts AS post WHERE post.title = $1::varchar",
            title
        )
        .fetch_one(self.pool)
        .await
        .map_err(|err| ApplicationLayerError::PersistenceError(Box::new(err)))
        .map(|record| record.count.unwrap_or_default() > 0)
    }
}
