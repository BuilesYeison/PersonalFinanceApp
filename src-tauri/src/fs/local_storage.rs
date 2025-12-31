use crate::domain::config_models::LastSessionCacheConfig;
use crate::domain::error::AppError;
use crate::helpers::json_helpers::save_json;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::AppHandle;
use tauri::Manager; // Para acceder a app_data_dir

pub struct LocalPaths {
    pub cache_dir: PathBuf,
    pub logs_dir: PathBuf,
    pub temp_dir: PathBuf,
}

/// Obtiene y crea la estructura local para un workspace específico
pub fn prepare_local_storage(
    app: &AppHandle,
    base_path: String,
    workspace_name: String,
) -> Result<LocalPaths, AppError> {
    let workspace_path = Path::new(&base_path).join(&workspace_name);
    // 1. Obtener la ruta base de la App (ej: AppData/Roaming/MyFinApp)
    let app_data:PathBuf = app.path().app_data_dir().map_err(|_| {
        AppError::IoError("No se pudo encontrar el directorio de datos de la app".into())
    })?;

    // 2. Construir MyFinApp/workspaces/NombreDelWorkspace/
    let base = app_data.join("workspaces").join(workspace_name.clone());

    let paths = LocalPaths {
        cache_dir: base.join("cache"),
        logs_dir: base.join("logs"),
        temp_dir: base.join("temp"),
    };

    // 3. Crear los directorios físicamente
    fs::create_dir_all(&paths.cache_dir)?;
    fs::create_dir_all(&paths.logs_dir)?;
    fs::create_dir_all(&paths.temp_dir)?;

    //json
    let session_file = app_data.join("last_session.json");
    save_json(
        session_file,
        &LastSessionCacheConfig {
            last_workspace_path: workspace_path.to_string_lossy().into(),
            last_workspace_name: workspace_name,
        },
    )?;

    Ok(paths)
}
