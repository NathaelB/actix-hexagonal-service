use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: u32,
    pub name: String,
}

impl Category {
    pub fn new(id: u32, name: String) -> Self {
        Self { id, name }
    }
}