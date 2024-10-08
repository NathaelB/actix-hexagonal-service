use async_trait::async_trait;
use crate::domain::entities::article::Article;

#[async_trait]
pub trait ArticleRepository: Send + Sync {
    async fn create(&self, article: Article) -> Option<Article>;
    async fn get_by_id(&self, id: u32) -> Option<Article>;
    async fn list(&self) -> Vec<Article>;
}