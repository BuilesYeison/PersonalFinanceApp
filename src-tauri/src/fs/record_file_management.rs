use crate::domain::config_models::RecordItem;
use crate::domain::error::AppError;
use crate::dto::create_record_dto::CreateRecordDto;
use crate::helpers::json_helpers::save_json;
use std::path::PathBuf;

pub fn create_record_file(
    workspace_path: &PathBuf,
    dto: &CreateRecordDto,
    record_id: &str,
) -> Result<(), AppError> {
    let records_dir = workspace_path.join("records");
    let record_file_path = records_dir.join(format!("{}.json", record_id));

    let record_item = RecordItem {
        id: record_id.to_string(),
        r#type: dto.r#type.clone(),
        amount: dto.amount,
        currency: None,
        timestamp: dto.timestamp,
        category_id: dto.category_id.clone(),
        account_id: dto.account_id.clone(),
        to_account_id: dto.to_account_id.clone(),
        description: dto.description.clone(),
        metadata: None,
    };

    save_json(record_file_path, &record_item)?;

    Ok(())
}
