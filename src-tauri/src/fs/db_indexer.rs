use crate::domain::config_models::{AccountsConfig, CategoriesConfig, RecordItem};
use crate::domain::error::AppError;
use rusqlite::{params, Connection};
use std::path::Path;
use std::{fs, path};

pub fn index_full_workspace(workspace_path: &Path, conn: &mut Connection) -> Result<(), AppError> {
    let tx = conn
        .transaction()
        .map_err(|e| AppError::IoError(e.to_string()))?;
    let finance_dir = workspace_path.join(".finance");

    // 1. Indexar Categorías
    let cat_json = fs::read_to_string(finance_dir.join("categories.json"))?;
    let cat_data: CategoriesConfig = serde_json::from_str(&cat_json)
        .map_err(|e| AppError::ConfigError(format!("Error en categorías: {}", e)))?;

    for cat in cat_data.categories {
        tx.execute(
            "INSERT OR REPLACE INTO categories (id, name, type, icon, color, is_active) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![cat.id, cat.name, cat.r#type, cat.icon, cat.color, cat.is_active],
        ).map_err(|e| AppError::IoError(e.to_string()))?;
    }

    // 2. Indexar Cuentas
    let acc_json = fs::read_to_string(finance_dir.join("accounts.json"))?;
    let acc_data: AccountsConfig = serde_json::from_str(&acc_json)
        .map_err(|e| AppError::ConfigError(format!("Error en cuentas: {}", e)))?;

    for acc in acc_data.accounts {
        tx.execute(
            "INSERT OR REPLACE INTO accounts (id, name, type, currency, initial_balance, is_active, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![acc.id, acc.name, acc.r#type, acc.currency, acc.initial_balance, acc.is_active, acc.created_at],
        ).map_err(|e| AppError::IoError(e.to_string()))?;
    }

    // 3. Indexar Transacciones (Carpeta records/)
    let records_dir = workspace_path.join("records");
    if records_dir.exists() {
        for entry in fs::read_dir(records_dir)? {
            let entry = entry?;
            let path = entry.path();

            // Solo procesamos archivos .json
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let rec_json = fs::read_to_string(&path)?;
                if let Ok(record) = serde_json::from_str::<RecordItem>(&rec_json) {
                    tx.execute(
                        "INSERT OR REPLACE INTO records (id, type, timestamp, amount, account_id, to_account_id, category_id, description, file_path) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                        params![record.id, record.r#type, record.timestamp, record.amount, record.account_id, record.to_account_id,record.category_id, record.description, path.to_str().unwrap()],
                    ).map_err(|e| AppError::IoError(e.to_string()))?;
                }
            }
        }
    }

    tx.commit().map_err(|e| AppError::IoError(e.to_string()))?;
    Ok(())
}
