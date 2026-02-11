use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryDto {
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub icon: String,
    pub color: String,
    pub is_active: bool,
}
