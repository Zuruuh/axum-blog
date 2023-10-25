use std::fmt;

mod post_repository;

pub use post_repository::PostRepository;

use super::{
    error::ApplicationLayerError,
    validation::{ConstraintViolation, ConstraintViolationLocation},
};

#[derive(Debug, serde::Serialize)]
pub struct Post {
    pub id: uuid::Uuid,
    pub title: String,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_update: chrono::DateTime<chrono::Utc>,
}

impl Post {
    pub fn new(create_post_dto: ValidatedCreatePostDTO) -> Self {
        let now = chrono::Utc::now();

        Self {
            id: create_post_dto.id.unwrap_or_else(uuid::Uuid::now_v7),
            title: create_post_dto.title.to_string(),
            content: create_post_dto.content,
            created_at: now,
            last_update: now,
        }
    }

    pub fn update_from(&mut self, update_post_dto: &ValidatedUpdatePostDTO) {
        self.title = update_post_dto.title.to_string();
        self.content = update_post_dto.content.clone();
        self.last_update = chrono::Utc::now();
    }
}

#[derive(Debug)]
pub struct PostTitle(String);

impl PostTitle {
    const TITLE_MAX_LEN: usize = 255;
    const TITLE_MIN_LEN: usize = 6;

    pub fn new(title: String) -> Result<Self, String> {
        if title.len() > Self::TITLE_MAX_LEN || title.len() < Self::TITLE_MIN_LEN {
            return Err(format!(
                "Your title's length must be between {} and {} chars",
                Self::TITLE_MIN_LEN,
                Self::TITLE_MAX_LEN
            ));
        }

        Ok(Self(title))
    }
}

impl fmt::Display for PostTitle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct CreatePostDTO {
    pub id: Option<uuid::Uuid>,
    pub title: String,
    pub content: String,
}

#[derive(Debug)]
pub struct ValidatedCreatePostDTO {
    pub id: Option<uuid::Uuid>,
    pub title: PostTitle,
    pub content: String,
}

impl ValidatedCreatePostDTO {
    pub async fn new(
        dto: CreatePostDTO,
        post_repository: &mut impl PostRepository,
    ) -> Result<Self, ApplicationLayerError> {
        let mut violations: Vec<ConstraintViolation> = vec![];
        if let Some(id) = dto.id {
            match post_repository.exists_with_id(&id).await {
                Ok(exists) => {
                    if exists {
                        violations.push(ConstraintViolation::new(
                            "A post with the given ID already exists!".into(),
                            "id".into(),
                            ConstraintViolationLocation::Body,
                        ))
                    }

                    Ok(())
                }
                Err(err) => Err(err),
            }?;
        }

        match post_repository.exists_with_title(&dto.title).await {
            Ok(exists) => {
                if exists {
                    violations.push(ConstraintViolation::new(
                        "A post with the given title already exists!".into(),
                        "title".into(),
                        ConstraintViolationLocation::Body,
                    ))
                }

                Ok(())
            }
            Err(err) => Err(err),
        }?;

        let title = PostTitle::new(dto.title);
        if let Err(title_err) = &title {
            violations.push(ConstraintViolation::new(
                title_err.clone(),
                "title".into(),
                ConstraintViolationLocation::Body,
            ));
        }

        if !violations.is_empty() {
            return Err(ApplicationLayerError::ValidationError(violations));
        }

        Ok(Self {
            id: dto.id,
            title: title.unwrap(),
            content: dto.content,
        })
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdatePostDTO {
    pub title: String,
    pub content: String,
}

pub struct ValidatedUpdatePostDTO {
    pub title: PostTitle,
    pub content: String,
}

impl ValidatedUpdatePostDTO {
    pub async fn new(
        dto: UpdatePostDTO,
        post: &Post,
        post_repository: &mut impl PostRepository,
    ) -> Result<Self, ApplicationLayerError> {
        let mut violations: Vec<ConstraintViolation> = vec![];

        if post.title.to_string() != dto.title {
            match post_repository.exists_with_title(&dto.title).await {
                Ok(exists) => {
                    if exists {
                        violations.push(ConstraintViolation::new(
                            "A post with the given title already exists!".into(),
                            "title".into(),
                            ConstraintViolationLocation::Body,
                        ))
                    }

                    Ok(())
                }
                Err(err) => Err(err),
            }?;
        }

        let title = PostTitle::new(dto.title);
        if let Err(title_err) = &title {
            violations.push(ConstraintViolation::new(
                title_err.clone(),
                "title".into(),
                ConstraintViolationLocation::Body,
            ));
        }

        if !violations.is_empty() {
            return Err(ApplicationLayerError::ValidationError(violations));
        }

        Ok(Self {
            title: title.unwrap(),
            content: dto.content,
        })
    }
}
