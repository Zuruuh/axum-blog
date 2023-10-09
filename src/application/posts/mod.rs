use crate::domain::{
    error::ApplicationLayerError,
    posts::{CreatePostDTO, Post, PostRepository, ValidatedCreatePostDTO},
};

pub async fn create_post(
    post_repository: &mut impl PostRepository,
    create_post_dto: CreatePostDTO,
) -> Result<Post, ApplicationLayerError> {
    let create_post_dto = ValidatedCreatePostDTO::new(create_post_dto, post_repository)
        .await
        .map_err(|violations| ApplicationLayerError::ValidationError(violations))?;

    post_repository.persist(create_post_dto).await
}
