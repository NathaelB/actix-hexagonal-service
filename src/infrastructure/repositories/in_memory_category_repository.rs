use std::sync::Mutex;
use async_trait::async_trait;
use crate::domain::entities::category::Category;
use crate::domain::repositories::category_repository::CategoryRepository;

pub struct InMemoryCategoryRepository {
    categories: Mutex<Vec<Category>>
}

impl InMemoryCategoryRepository {
    pub fn new() -> Self {
        Self {
            categories: Mutex::new(vec![])
        }
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