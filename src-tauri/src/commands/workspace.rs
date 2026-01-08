use crate::domain::error::AppError;
use crate::fs::{db_indexer, db_init, local_storage, workspace_init};
use rusqlite::Connection;
use tauri::AppHandle;
use std::path::Path;

#[tauri::command]
pub async fn init_workspace(
    app: AppHandle,
    base_path: String,
    name: String,
) -> Result<String, AppError> {
    // 1. Crear el Workspace sincronizable (los JSON)
    workspace_init::init(base_path.clone(), name.clone())?;

    // 2. Crear el almacenamiento local (SQLite, Logs)
    let local_paths = local_storage::prepare_local_storage(&app, base_path.clone(), name.clone())?;

    // 3. Inicializar SQLite en la carpeta de cache recién creada

    let db_path = local_paths.cache_dir.join("cache.sqlite");
    let mut conn = Connection::open(db_path).map_err(|e| AppError::IoError(e.to_string()))?;
    db_init::init_sqlite(&mut conn).map_err(|e| AppError::IoError(format!("Error DB: {}", e)))?;

    // 4. Indexar
    db_indexer::index_full_workspace(&std::path::Path::new(&base_path).join(&name), &mut conn)?;

    Ok("Workspace e índice local creados correctamente".into())
}

#[tauri::command]
pub async fn open_workspace(app: AppHandle, full_path: String) -> Result<String, AppError> {
    let path = Path::new(&full_path);

    if (!path.exists() || !path.is_dir()) {
        return Err(AppError::IoError(
            "La ruta seleccionada no es un directorio válido".into(),
        ));
    }

    let finance_dir = path.join(".finance");
    if (!finance_dir.exists()
        || !finance_dir.join("app.json").exists()
        || !finance_dir.join("version.json").exists())
    {
        return Err(AppError::IoError(
            "La carpeta seleccionada no parece ser un workspace de MyFinApp (falta .finance/app.json)".into()
        ));
    }

    let workspace_name = path
        .file_name()
        .map(|n: &std::ffi::OsStr| n.to_string_lossy().into_owned())
        .ok_or_else(|| AppError::IoError("No se pudo determinar el nombre de la carpeta".into()))?;

    let local_paths =
        local_storage::prepare_local_storage(&app, full_path.clone(), workspace_name.clone())?;

    let db_path = local_paths.cache_dir.join("cache.sqlite");
    let mut conn = Connection::open(&db_path)
        .map_err(|e| AppError::IoError(format!("Error al abrir DB: {}", e)))?;

    // Aseguramos que las tablas existan (por si es una instalación limpia)
    db_init::init_sqlite(&mut conn)
        .map_err(|e| AppError::IoError(format!("Error al inicializar tablas: {}", e)))?;

    // Ejecutamos el indexador que lee todos los JSON y llena el SQLite
    db_indexer::index_full_workspace(path, &mut conn)?;

    Ok(format!("Workspace '{}' abierto e indexado", workspace_name))
}
