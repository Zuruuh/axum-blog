use std::fmt;

mod post_repository;

pub use post_repository::PostRepository;

use super::validation::ConstraintViolation;

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
}

#[derive(Debug)]
pub struct PostTitle(String);

impl PostTitle {
    const TITLE_MAX_LEN: usize = 255;
    const TITLE_MIN_LEN: usize = 6;

    pub fn new(title: String) -> Result<Self, String> {
        if title.len() > Self::TITLE_MAX_LEN || title.len() < Self::TITLE_MIN_LEN {
            return Err("".into());
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
    ) -> Result<Self, Vec<ConstraintViolation>> {
        let mut violations: Vec<ConstraintViolation> = vec![];
        if let Some(id) = dto.id {
            match post_repository.exists_with_id(&id).await {
                Ok(exists) => {
                    if exists {
                        violations.push(ConstraintViolation::new(
                            "A post with the given ID already exists!".into(),
                            "id".into(),
                        ))
                    }
                }
                Err(_) => unimplemented!(),
            }
        }

        match post_repository.exists_with_title(&dto.title).await {
            Ok(exists) => {
                if exists {
                    violations.push(ConstraintViolation::new(
                        "A post with the given title already exists!".into(),
                        "title".into(),
                    ))
                }
            }
            Err(_) => unimplemented!(),
        }

        let title = PostTitle::new(dto.title);
        if let Err(title_err) = &title {
            violations.push(ConstraintViolation::new(title_err.clone(), "title".into()));
        }

        if !violations.is_empty() {
            return Err(violations);
        }

        Ok(Self {
            id: dto.id,
            title: title.unwrap(),
            content: dto.content,
        })
    }
}
