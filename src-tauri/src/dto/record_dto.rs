use serde::{Deserialize, Serialize};
use super::{account_info_dto::AccountInfoDto, category_dto::CategoryDto};

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordDto {
    pub id: String,
    pub r#type: String,
    pub amount: f64,
    pub currency: Option<String>,
    pub timestamp: i64,
    pub description: Option<String>,
    pub account: Option<AccountInfoDto>,
    pub to_account: Option<AccountInfoDto>,
    pub category: Option<CategoryDto>,
}
