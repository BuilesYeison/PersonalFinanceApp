use crate::dto::account_info_dto::AccountInfoDto;
use crate::dto::pagination_dto::Pagination;
use crate::dto::record_dto::RecordDto;
use crate::fs::account_file_management::{add_account_to_list, remove_account_from_list, update_account_in_json};
use crate::services::accounts::{create_account_in_database, delete_account_if_no_records, update_account_in_database};
use crate::services::records::get_records;
use crate::services::{accounts, stats};
use crate::AppState;
use crate::{domain::error::AppError, services::stats::DashboardStats};

#[tauri::command]
pub async fn get_overall_stats(
    state: tauri::State<'_, AppState>,
) -> Result<DashboardStats, AppError> {
    // 1. Bloqueamos el Mutex para obtener el Guard
    let mut conn_guard = state.db.lock().unwrap();

    // 2. Usamos .as_mut() para obtener una referencia mutable a la conexión
    // El error principal en tu código era intentar desestructurar con &mut conn directamente
    let conn = conn_guard.as_mut().ok_or_else(|| {
        AppError::DatabaseError("No hay una conexión a la base de datos activa".into())
    })?;

    let overall_stats =
        stats::calculate_overall_stats(conn).map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(overall_stats)
}

#[tauri::command]
pub async fn get_accounts(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<AccountInfoDto>, AppError> {
    // 1. Bloqueamos el Mutex para obtener el Guard
    let mut conn_guard = state.db.lock().unwrap();

    // 2. Usamos .as_mut() para obtener una referencia mutable a la conexión
    // El error principal en tu código era intentar desestructurar con &mut conn directamente
    let conn = conn_guard.as_mut().ok_or_else(|| {
        AppError::DatabaseError("No hay una conexión a la base de datos activa".into())
    })?;
    let accounts = accounts::get_accounts_with_balance(conn)
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(accounts)
}

#[tauri::command]
pub async fn create_account(
    state: tauri::State<'_, AppState>,
    new_account: AccountInfoDto,
) -> Result<String, AppError> {
    // 1. Bloqueamos el Mutex para obtener el Guard
    let mut conn_guard = state.db.lock().unwrap();

    // 2. Usamos .as_mut() para obtener una referencia mutable a la conexión
    // El error principal en tu código era intentar desestructurar con &mut conn directamente
    let conn = conn_guard.as_mut().ok_or_else(|| {
        AppError::DatabaseError("No hay una conexión a la base de datos activa".into())
    })?;

    let path_guard = state.workspace_path.lock().unwrap();
    let workspace_path = path_guard.as_ref().ok_or(AppError::IoError(
        "No hay un workspace activo en el estado".into(),
    ))?;

    // 2. Construir la ruta al archivo
    let accounts_path = workspace_path.join(".finance").join("accounts.json");

    let account_id = uuid::Uuid::new_v4().to_string();

    add_account_to_list(accounts_path, &new_account, &account_id)
        .map_err(|e| AppError::IoError(format!("Error al agregar cuenta en archivos json: {}", e)));
    create_account_in_database(conn, new_account, &account_id)
        .map_err(|e| AppError::DatabaseError(format!("Error creando cuenta en DB: {}", e)))?;

    Ok(account_id.into())
}

#[tauri::command]
pub async fn update_account(
    state: tauri::State<'_, AppState>,
    account: AccountInfoDto,
) -> Result<(), AppError> {
    println!("Received account to update: {:?}", account);
    let mut conn_guard = state.db.lock().unwrap();
    let conn = conn_guard.as_mut().ok_or_else(|| {
        AppError::DatabaseError("No hay una conexión a la base de datos activa".into())
    })?;

    let path_guard = state.workspace_path.lock().unwrap();
    let workspace_path = path_guard.as_ref().ok_or(AppError::IoError(
        "No hay un workspace activo en el estado".into(),
    ))?;

    let accounts_path = workspace_path.join(".finance").join("accounts.json");

    update_account_in_json(accounts_path, &account)?;
    update_account_in_database(conn, &account)?;

    Ok(())
}


#[tauri::command]
pub async fn delete_account(
    state: tauri::State<'_, AppState>,
    account_id: String,
) -> Result<bool, AppError> {
    // 1. Bloqueamos el Mutex para obtener el Guard
    let mut conn_guard = state.db.lock().unwrap();

    // 2. Usamos .as_mut() para obtener una referencia mutable a la conexión
    // El error principal en tu código era intentar desestructurar con &mut conn directamente
    let conn = conn_guard.as_mut().ok_or_else(|| {
        AppError::DatabaseError("No hay una conexión a la base de datos activa".into())
    })?;

    let path_guard = state.workspace_path.lock().unwrap();
    let workspace_path = path_guard.as_ref().ok_or(AppError::IoError(
        "No hay un workspace activo en el estado".into(),
    ))?;

    // 2. Construir la ruta al archivo
    let accounts_path = workspace_path.join(".finance").join("accounts.json");

    let result: bool = delete_account_if_no_records(conn, &account_id)
        .map_err(|e| AppError::DatabaseError(format!("Error eliminando cuenta en DB: {}", e)))?;

    if result {
        remove_account_from_list(accounts_path, &account_id).map_err(|e| {
            AppError::IoError(format!("Error al eliminar cuenta en archivos json: {}", e))
        })?;
    }

    Ok(result)
}

#[tauri::command]
pub async fn get_paginated_records(
    state: tauri::State<'_, AppState>,
    page: i16,
    size: i16,
) -> Result<Pagination<RecordDto>, AppError> {
    // 1. Bloqueamos el Mutex para obtener el Guard
    let mut conn_guard = state.db.lock().unwrap();

    // 2. Usamos .as_mut() para obtener una referencia mutable a la conexión
    // El error principal en tu código era intentar desestructurar con &mut conn directamente
    let conn = conn_guard.as_mut().ok_or_else(|| {
        AppError::DatabaseError("No hay una conexión a la base de datos activa".into())
    })?;

    let result: Pagination<RecordDto> = get_records(conn, page, size)
        .map_err(|e| AppError::DatabaseError(format!("Error obteniendo registros: {}", e)))?;
    Ok(result)
}
