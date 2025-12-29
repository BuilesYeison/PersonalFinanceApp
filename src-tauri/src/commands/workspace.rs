use crate::fs::workspace_init;

#[tauri::command]
pub fn init_workspace(base_path: String, name: String) -> Result<String, String> {
    workspace_init::init(base_path, name).map_err(|e| e.to_string())?;

    Ok("Workspace creado correctamente".into())
}
