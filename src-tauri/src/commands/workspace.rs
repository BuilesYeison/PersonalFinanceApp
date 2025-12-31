use crate::domain::error::AppError;
use crate::fs::{local_storage, workspace_init};
use tauri::AppHandle;

#[tauri::command]
pub async fn init_workspace(
    app: AppHandle,
    base_path: String,
    name: String,
) -> Result<String, AppError> {
    // 1. Crear el Workspace sincronizable (los JSON)
    workspace_init::init(base_path.clone(), name.clone())?;

    // 2. Crear el almacenamiento local (SQLite, Logs)
    let local_paths = local_storage::prepare_local_storage(&app, base_path, name.clone())?;

    // 3. TODO: Inicializar la DB SQLite en local_paths.cache_dir
    // init_sqlite(&local_paths.cache_dir.join("cache.sqlite"))?;

    Ok("Workspace e índice local creados correctamente".into())
}
