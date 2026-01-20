use crate::dto::account_info_dto::AccountInfoDto;
use crate::services::{stats,accounts};
use crate::{domain::error::AppError, services::stats::DashboardStats};
use crate::AppState;

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
pub async fn get_accounts(state: tauri::State<'_, AppState>,) -> Result<Vec<AccountInfoDto>, AppError> {
    // 1. Bloqueamos el Mutex para obtener el Guard
    let mut conn_guard = state.db.lock().unwrap();

    // 2. Usamos .as_mut() para obtener una referencia mutable a la conexión
    // El error principal en tu código era intentar desestructurar con &mut conn directamente
    let conn = conn_guard.as_mut().ok_or_else(|| {
        AppError::DatabaseError("No hay una conexión a la base de datos activa".into())
    })?;
    let accounts = accounts::get_accounts_with_balance(conn).map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(accounts)
}