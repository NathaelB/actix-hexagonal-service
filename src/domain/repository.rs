use crate::domain::article::Article;
use async_trait::async_trait;
use crate::domain::category::Category;

#[async_trait]
pub trait ArticleRepository: Send + Sync {
    async fn create(&self, article: Article) -> Option<Article>;
    async fn get_by_id(&self, id: u32) -> Option<Article>;
    async fn list(&self) -> Vec<Article>;
}


#[async_trait]
pub trait CategoryRepository: Send + Sync {
    async fn create(&self, category: Category) -> Option<Category>;
    #[allow(unused)]
    async fn get_by_id(&self, id: u32) -> Option<Category>;
    async fn list(&self) -> Vec<Category>;
}