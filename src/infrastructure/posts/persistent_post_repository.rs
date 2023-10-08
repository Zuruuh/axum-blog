use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::domain::{
    persistence::PersistenceError,
    posts::{CreatePostDTO, Post, PostRepository},
};

#[derive(Clone)]
pub struct PersistentPostRepository<'a> {
    pool: &'a Pool<Postgres>,
}

impl<'a> PersistentPostRepository<'a> {
    pub fn new(pool: &'a Pool<Postgres>) -> Self { Self { pool } }
}

#[async_trait]
impl<'a> PostRepository for PersistentPostRepository<'a> {
    async fn persist(&mut self, create_post_dto: CreatePostDTO) -> Result<Post, PersistenceError> {
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
        .map_err(|err| PersistenceError::UncheckedError(Box::new(err)))?;

        Ok(post)
    }
}
