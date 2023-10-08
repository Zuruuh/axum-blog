mod post_repository;

pub use post_repository::PostRepository;

#[derive(Debug, serde::Serialize)]
pub struct Post {
    pub id: uuid::Uuid,
    pub title: String,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_update: chrono::DateTime<chrono::Utc>,
}

impl Post {
    pub fn new(create_post_dto: CreatePostDTO) -> Self {
        let now = chrono::Utc::now();

        Self {
            id: create_post_dto.id.unwrap_or_else(uuid::Uuid::new_v4),
            title: create_post_dto.title,
            content: create_post_dto.content,
            created_at: now,
            last_update: now,
        }
    }
}

#[derive(serde::Deserialize)]
pub struct CreatePostDTO {
    pub id: Option<uuid::Uuid>,
    pub title: String,
    pub content: String,
}
