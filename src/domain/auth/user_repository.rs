use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn create();
}
