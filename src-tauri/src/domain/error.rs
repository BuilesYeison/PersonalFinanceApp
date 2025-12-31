use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize)]
#[serde(tag = "type", content = "message")]
pub enum AppError {
    #[error("La carpeta ya existe: {0}")]
    WorkspaceExists(String),

    #[error("Error de sistema de archivos: {0}")]
    IoError(String),

    #[error("No se pudo crear el archivo de configuración: {0}")]
    ConfigError(String),
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::IoError(error.to_string())
    }
}
