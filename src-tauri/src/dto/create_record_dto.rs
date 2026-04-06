use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRecordDto {
    #[serde(rename = "type")]
    pub r#type: String,
    pub amount: f64,
    pub account_id: String,
    pub to_account_id: Option<String>,
    pub category_id: Option<String>,
    pub description: Option<String>,
    pub timestamp: i64,
}
