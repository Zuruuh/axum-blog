use crate::domain::{
    error::ApplicationLayerError,
    pagination::PaginationOptions,
    posts::{CreatePostDTO, Post, PostRepository, ValidatedCreatePostDTO},
};

pub async fn create_post(
    post_repository: &mut impl PostRepository,
    create_post_dto: CreatePostDTO,
) -> Result<Post, ApplicationLayerError> {
    let create_post_dto = ValidatedCreatePostDTO::new(create_post_dto, post_repository).await?;

    post_repository.persist(create_post_dto).await
}

pub async fn list_posts(
    post_repository: &impl PostRepository,
    pagination_options: &PaginationOptions,
) -> Result<Vec<Post>, ApplicationLayerError> {
    post_repository
        .list_posts(
            &pagination_options.skip.unwrap_or(0),
            &pagination_options.take.unwrap_or(10),
        )
        .await
}

pub async fn find_post(
    post_repository: &impl PostRepository,
    post_id: &uuid::Uuid,
) -> Result<Option<Post>, ApplicationLayerError> {
    post_repository.find_post(post_id).await
}

pub async fn delete_post(
    post_repository: &mut impl PostRepository,
    post_id: &uuid::Uuid,
) -> Result<bool, ApplicationLayerError> {
    post_repository.delete_post(post_id).await
}
