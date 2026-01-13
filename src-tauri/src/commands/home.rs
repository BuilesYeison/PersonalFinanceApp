use crate::services::stats;
use crate::{domain::error::AppError, services::stats::DashboardStats};
use rusqlite::Connection;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Manager;

#[tauri::command]
pub async fn get_overall_stats(
    app: AppHandle,
    workspace_name: String,
) -> Result<DashboardStats, AppError> {
    let app_data: PathBuf = app.path().app_data_dir().map_err(|_| {
        AppError::IoError("No se pudo encontrar el directorio de datos de la app".into())
    })?;
    let base = app_data.join("workspaces").join(workspace_name.clone());

    let db_path = base.join("cache").join("cache.sqlite");
    let mut conn = Connection::open(db_path).map_err(|e| AppError::IoError(e.to_string()))?;

    let overall_stats =
        stats::calculate_overall_stats(&mut conn).map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(overall_stats)
}
