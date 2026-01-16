use crate::domain::error::AppError;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

pub fn save_json<T: Serialize>(path: PathBuf, data: &T) -> Result<(), AppError> {
    let contents = serde_json::to_string_pretty(data)
        .map_err(|e| AppError::ConfigError(format!("Error serializando: {}", e)))?;

    fs::write(&path, contents)
        .map_err(|e| AppError::ConfigError(format!("Error escribiendo en {:?}: {}", path, e)))?;

    Ok(())
}

/// Lee un archivo JSON y lo transforma en el Struct que especifiques
pub fn load_json<T: DeserializeOwned>(path: &Path) -> Result<T, AppError> {
    // 1. Leer el archivo a un String
    let content = fs::read_to_string(path)
        .map_err(|e| AppError::IoError(format!("No se pudo leer el archivo {:?}: {}", path, e)))?;

    // 2. Parsear el String a la estructura T
    let data: T = serde_json::from_str(&content).map_err(|e| {
        AppError::ConfigError(format!("Error al parsear JSON en {:?}: {}", path, e))
    })?;

    Ok(data)
}
