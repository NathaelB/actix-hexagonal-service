use async_trait::async_trait;
use crate::domain::entities::article::Article;

#[async_trait]
pub trait ArticleService {
    async fn create_article(&self, title: String, content: String) -> Option<Article>;
    async fn get_article(&self, id: u32) -> Option<Article>;
    async fn list_articles(&self) -> Vec<Article>;
}