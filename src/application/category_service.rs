use crate::domain::category::Category;
use crate::domain::repository::CategoryRepository;

pub struct CategoryService {
    repository: Box<dyn CategoryRepository + Send + Sync>
}

impl CategoryService {
    pub fn new(repository: Box<dyn CategoryRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn create_category(&self, name: String) -> Option<Category> {
        let new_id = (self.repository.list().await.len() as u32) + 1;
        let category = Category::new(new_id, name);

        self.repository.create(category).await
    }

    #[allow(unused)]
    pub async fn get_category(&self, id: u32) -> Option<Category> {
        self.repository.get_by_id(id).await
    }

    pub async fn list_categories(&self) -> Vec<Category> {
        self.repository.list().await
    }
}