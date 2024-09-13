use crate::domain::article::Article;
use crate::domain::repository::{ArticleRepository, CategoryRepository};
use async_trait::async_trait;
use std::sync::Mutex;
use crate::domain::category::Category;

pub struct InMemoryArticleRepository {
    articles: Mutex<Vec<Article>>,
}

pub struct InMemoryCategoryRepository {
    categories: Mutex<Vec<Category>>,
}

impl InMemoryArticleRepository {
    pub fn new() -> Self {
        Self {
            articles: Mutex::new(vec![]),
        }
    }
}

impl InMemoryCategoryRepository {
    pub fn new() -> Self {
        Self {
            categories: Mutex::new(vec![]),
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

#[async_trait]
impl CategoryRepository for InMemoryCategoryRepository {
    async fn create(&self, category: Category) -> Option<Category> {
        let mut categories = self.categories.lock().unwrap();
        categories.push(category.clone());
        Some(category)
    }

    async fn get_by_id(&self, id: u32) -> Option<Category> {
        let categories = self.categories.lock().unwrap();
        categories.iter().find(|&c| c.id == id).cloned()
    }

    async fn list(&self) -> Vec<Category> {
        let categories = self.categories.lock().unwrap();
        categories.clone()
    }
}