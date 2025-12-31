use crate::fs::workspace_init;
use crate::domain::error::AppError;

#[tauri::command]
pub async fn init_workspace(base_path: String, name: String) -> Result<String, AppError> {
    workspace_init::init(base_path, name)?;
    Ok("¡Workspace creado con éxito!".into())
}