use crate::domain::{
    error::ApplicationError,
    pagination::PaginationOptions,
    posts::{
        CreatePostDTO, Post, PostRepository, UpdatePostDTO, ValidatedCreatePostDTO,
        ValidatedUpdatePostDTO,
    },
    validation::ConstraintViolation,
};

pub async fn create_post(
    post_repository: &mut impl PostRepository,
    create_post_dto: CreatePostDTO,
) -> Result<Post, ApplicationError> {
    let create_post_dto = ValidatedCreatePostDTO::new(create_post_dto, post_repository).await?;

    post_repository.persist(create_post_dto).await
}

pub async fn list_posts(
    post_repository: &impl PostRepository,
    pagination_options: &PaginationOptions,
) -> Result<Vec<Post>, ApplicationError> {
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
) -> Result<Option<Post>, ApplicationError> {
    post_repository.find_post(post_id).await
}

pub async fn delete_post(
    post_repository: &mut impl PostRepository,
    post_id: &uuid::Uuid,
) -> Result<bool, ApplicationError> {
    post_repository.delete_post(post_id).await
}

pub async fn update_post(
    post_repository: &mut impl PostRepository,
    post_id: &uuid::Uuid,
    update_post_dto: UpdatePostDTO,
) -> Result<Post, ApplicationError> {
    let mut post =
        post_repository
            .find_post(post_id)
            .await?
            .ok_or(ApplicationError::ValidationError(vec![
                ConstraintViolation::new(
                    format!("Could not find post with id {post_id}!"),
                    "post_id".into(),
                    crate::domain::validation::ConstraintViolationLocation::Path,
                ),
            ]))?;
    let update_post_dto =
        ValidatedUpdatePostDTO::new(update_post_dto, &post, post_repository).await?;
    post.update_from(&update_post_dto);

    post_repository.update_post(&post).await?;

    Ok(post)
}
