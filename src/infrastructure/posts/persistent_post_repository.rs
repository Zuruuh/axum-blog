use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::domain::{
    error::ApplicationError,
    posts::{Post, PostRepository, ValidatedCreatePostDTO},
};

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
    ) -> Result<Post, ApplicationError> {
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
        .map_err(|err| ApplicationError::PersistenceError(Box::new(err)))?;

        Ok(post)
    }

    async fn exists_with_id(&self, id: &uuid::Uuid) -> Result<bool, ApplicationError> {
        sqlx::query!(
            "SELECT COUNT(*) FROM app_posts AS post WHERE post.id = $1::uuid",
            id
        )
        .fetch_one(self.pool)
        .await
        .map_err(|err| ApplicationError::PersistenceError(Box::new(err)))
        .map(|record| record.count.unwrap_or_default() > 0)
    }

    async fn exists_with_title(&self, title: &String) -> Result<bool, ApplicationError> {
        sqlx::query!(
            "SELECT COUNT(*) FROM app_posts AS post WHERE post.title = $1::varchar",
            title
        )
        .fetch_one(self.pool)
        .await
        .map_err(|err| ApplicationError::PersistenceError(Box::new(err)))
        .map(|record| record.count.unwrap_or_default() > 0)
    }

    async fn list_posts(&self, skip: &i32, take: &i32) -> Result<Vec<Post>, ApplicationError> {
        sqlx::query_as!(
            Post,
            "SELECT * FROM app_posts AS post LIMIT $1::int OFFSET $2::int",
            take,
            skip
        )
        .fetch_all(self.pool)
        .await
        .map_err(|err| ApplicationError::PersistenceError(Box::new(err)))
    }

    async fn find_post(&self, id: &uuid::Uuid) -> Result<Option<Post>, ApplicationError> {
        sqlx::query_as!(
            Post,
            "SELECT * FROM app_posts AS post WHERE id = $1::uuid",
            id
        )
        .fetch_optional(self.pool)
        .await
        .map_err(|err| ApplicationError::PersistenceError(Box::new(err)))
    }

    async fn delete_post(&mut self, id: &uuid::Uuid) -> Result<bool, ApplicationError> {
        sqlx::query!("DELETE FROM app_posts WHERE id = $1::uuid", id)
            .execute(self.pool)
            .await
            .map_err(|err| ApplicationError::PersistenceError(Box::new(err)))
            .map(|record| record.rows_affected() == 1)
    }

    async fn update_post(&mut self, post: &Post) -> Result<bool, ApplicationError> {
        sqlx::query!(
            "UPDATE app_posts SET title = $1::varchar, content = $2::text, last_update = $3::timestamptz WHERE id = $4::uuid",
            post.title,
            post.content,
            post.last_update,
            post.id,
        )
        .execute(self.pool)
        .await
        .map_err(|err| ApplicationError::PersistenceError(Box::new(err)))
        .map(|record| record.rows_affected() == 1)
    }
}
