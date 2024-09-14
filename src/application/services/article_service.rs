use async_trait::async_trait;
use crate::application::ports::article_service::ArticleService;
use crate::domain::entities::article::Article;
use crate::domain::repositories::article_repository::ArticleRepository;

pub struct ArticleServiceImpl {
    repository: Box<dyn ArticleRepository + Send + Sync>,
}

impl ArticleServiceImpl {
    pub fn new(repository: Box<dyn ArticleRepository + Send + Sync>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl ArticleService for ArticleServiceImpl {
    async fn create_article(&self, title: String, content: String) -> Option<Article> {
        let new_id = (self.repository.list().await.len() as u32) + 1;
        let article = Article::new(new_id, title, content);

        self.repository.create(article).await
    }

    async fn get_article(&self, id: u32) -> Option<Article> {
        self.repository.get_by_id(id).await
    }

    async fn list_articles(&self) -> Vec<Article> {
        self.repository.list().await
    }
}