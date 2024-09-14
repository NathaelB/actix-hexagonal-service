use async_trait::async_trait;
use crate::application::ports::category_service::CategoryService;
use crate::domain::entities::category::Category;
use crate::domain::repositories::category_repository::CategoryRepository;

pub struct CategoryServiceImpl {
    repository: Box<dyn CategoryRepository>
}

impl CategoryServiceImpl {
    pub fn new(repository: Box<dyn CategoryRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl CategoryService for CategoryServiceImpl {
    async fn create(&self, name: String) -> Option<Category> {
        let new_id = (self.repository.list().await.len() as u32) + 1;
        let category = Category::new(new_id, name);
        self.repository.create(category).await
    }

    async fn get_by_id(&self, id: u32) -> Option<Category> {
        self.repository.get_by_id(id).await
    }

    async fn list(&self) -> Vec<Category> {
        self.repository.list().await
    }
}