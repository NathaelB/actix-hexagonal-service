use async_trait::async_trait;
use crate::domain::entities::category::Category;

#[async_trait]
pub trait CategoryService {
    async fn create(&self, name: String) -> Option<Category>;
    async fn get_by_id(&self, id: u32) -> Option<Category>;
    async fn list(&self) -> Vec<Category>;
}