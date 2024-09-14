use std::sync::Mutex;
use async_trait::async_trait;
use crate::domain::entities::article::Article;
use crate::domain::repositories::article_repository::ArticleRepository;

pub struct InMemoryArticleRepository {
    pub articles: Mutex<Vec<Article>>
}

impl InMemoryArticleRepository {
    pub fn new() -> Self {
        Self {
            articles: Mutex::new(vec![])
        }
    }
}


#[async_trait]
impl ArticleRepository for InMemoryArticleRepository {
    async fn create(&self, article: Article) -> Option<Article> {
        let mut articles = self.articles.lock().unwrap();
        articles.push(article.clone());
        Some(article)
    }

    async fn get_by_id(&self, id: u32) -> Option<Article> {
        let articles = self.articles.lock().unwrap();
        articles.iter().find(|&a| a.id == id).cloned()
    }

    async fn list(&self) -> Vec<Article> {
        let articles = self.articles.lock().unwrap();
        articles.clone()
    }
}