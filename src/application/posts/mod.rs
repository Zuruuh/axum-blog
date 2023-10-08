use crate::domain::{
    persistence::PersistenceError,
    posts::{CreatePostDTO, Post, PostRepository},
};

pub async fn create_post(
    post_repository: &mut impl PostRepository,
    create_post_dto: CreatePostDTO,
) -> Result<Post, PersistenceError> {
    post_repository.persist(create_post_dto).await
}
