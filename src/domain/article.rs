use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: u32,
    pub title: String,
    pub content: String,
}

impl Article {
    pub fn new(id: u32, title: String, content: String) -> Self {
        Self { id, title, content }
    }
}
