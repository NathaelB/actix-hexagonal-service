use crate::domain::article::Article;
use crate::domain::repository::ArticleRepository;

pub struct ArticleService {
    repository: Box<dyn ArticleRepository + Send + Sync>,
}

impl ArticleService {
    pub fn new(repository: Box<dyn ArticleRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn create_article(&self, title: String, content: String) -> Option<Article> {
        let new_id = (self.repository.list().await.len() as u32) + 1;
        let article = Article::new(new_id, title, content);

        self.repository.create(article).await
    }

    pub async fn get_article(&self, id: u32) -> Option<Article> {
        self.repository.get_by_id(id).await
    }

    pub async fn list_articles(&self) -> Vec<Article> {
        self.repository.list().await
    }
}
