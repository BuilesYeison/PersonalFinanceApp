use serde::Serialize;
use std::path::PathBuf;
use crate::domain::error::AppError;
use std::fs;

pub fn save_json<T: Serialize>(path: PathBuf, data: &T) -> Result<(), AppError> {
    let contents = serde_json::to_string_pretty(data)
        .map_err(|e| AppError::ConfigError(format!("Error serializando: {}", e)))?;

    fs::write(&path, contents)
        .map_err(|e| AppError::ConfigError(format!("Error escribiendo en {:?}: {}", path, e)))?;

    Ok(())
}
