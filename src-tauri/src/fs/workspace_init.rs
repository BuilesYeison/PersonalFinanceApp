use crate::domain::config_models::*;
use crate::domain::error::AppError;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

pub fn init(base_path: String, name: String) -> Result<(), AppError> {
    // 1. Construir ruta del workspace
    let workspace_path = Path::new(&base_path).join(&name);

    // 2. Validar que no exista
    if workspace_path.exists() {
        return Err(AppError::WorkspaceExists(name));
    }

    if let Err(e) = create_structure(&workspace_path) {
        // ROLLBACK: Si algo falló, intentamos borrar la carpeta principal
        let _ = fs::remove_dir_all(&workspace_path);
        return Err(e);
    }

    Ok(())
}

fn create_structure(root: &PathBuf) -> Result<(), AppError> {
    fs::create_dir_all(root)?;
    // Carpetas
    let dirs = [".finance", "records", "attachments"];
    for dir in dirs {
        fs::create_dir(root.join(dir))?;
    }
    let config_dir = root.join(".finance");

    // Guardar archivos usando los nuevos modelos
    save_json(config_dir.join("version.json"), &VersionConfig::default())?;
    save_json(config_dir.join("app.json"), &AppConfig::default())?;
    save_json(
        config_dir.join("categories.json"),
        &CategoriesConfig::default(),
    )?;
    save_json(config_dir.join("accounts.json"), &AccountsConfig::default())?;

    // Archivos adicionales vacíos
    save_json(config_dir.join("budgets.json"), &serde_json::json!({}))?;
    save_json(config_dir.join("tags.json"), &serde_json::json!([]))?;

    Ok(())
}

// -------- helpers --------

fn save_json<T: Serialize>(path: PathBuf, data: &T) -> Result<(), AppError> {
    let contents = serde_json::to_string_pretty(data)
        .map_err(|e| AppError::ConfigError(format!("Error serializando: {}", e)))?;

    fs::write(&path, contents)
        .map_err(|e| AppError::ConfigError(format!("Error escribiendo en {:?}: {}", path, e)))?;

    Ok(())
}
