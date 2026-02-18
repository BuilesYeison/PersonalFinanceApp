use crate::domain::error::AppError;
use crate::helpers::json_helpers::{load_json, save_json};
use std::path::PathBuf;
use crate::domain::config_models::{AccountItem,AccountsConfig};
use crate::dto::account_info_dto::AccountInfoDto;
use crate::helpers::datetime_helpers::timestamp_now;


// Función para agregar una cuenta a la lista
pub fn add_account_to_list(file_path: PathBuf, new_account: &AccountInfoDto, account_id: &str) -> Result<(), AppError> {
    // 2. Cargar la lista existente
    // Verificamos si el archivo existe primero para evitar errores de IO si es la primera vez
    let mut accounts_config: AccountsConfig = if file_path.exists() {
        load_json(&file_path)?
    } else {
        AccountsConfig::default()
    };

    let new_account_item = AccountItem {
        id: account_id.clone().to_string(),
        name: new_account.name.clone(),
        r#type: new_account.account_type.clone().unwrap_or_default().clone(),
        currency: new_account.currency.clone().unwrap_or_default().clone(), // Usamos la moneda del workspace
        initial_balance: new_account.initial_balance.unwrap_or_default().clone(),
        credit_limit: new_account.credit_limit.clone(), // El formulario básico no lo pide, se deja en None
        is_active: true,
        created_at: timestamp_now(),
    };

    // 3. Agregar el nuevo objeto a la lista en memoria
    accounts_config.accounts.push(new_account_item);

    // 4. Guardar la lista actualizada en el archivo
    save_json(file_path, &accounts_config)?;

    Ok(())
}

pub fn update_account_in_json(
    file_path: PathBuf,
    updated_account: &AccountInfoDto,
) -> Result<(), AppError> {

    if !file_path.exists() {
        return Err(AppError::IoError("Archivo accounts.json no existe".into()));
    }

    let mut accounts_config: AccountsConfig = load_json(&file_path)?;

    let account = accounts_config
        .accounts
        .iter_mut()
        .find(|a| a.id == updated_account.id)
        .ok_or_else(|| AppError::NotFound(format!("Cuenta {} no encontrada", updated_account.id)))?;

    // Mutamos solo campos editables
    account.name = updated_account.name.clone();
    
    if let Some(account_type) = &updated_account.account_type {
        account.r#type = account_type.clone();
    }

    if let Some(currency) = &updated_account.currency {
        account.currency = currency.clone();
    }

    if let Some(initial_balance) = updated_account.initial_balance {
        account.initial_balance = initial_balance;
    }

    if let Some(credit_limit) = updated_account.credit_limit.clone() {
        account.credit_limit = Some(credit_limit);
    }

    save_json(file_path, &accounts_config)?;

    Ok(())
}



pub fn remove_account_from_list(file_path: PathBuf, account_id_to_remove: &str) -> Result<(), AppError> {
    // 1. Cargar la lista existente del archivo JSON.
    let mut accounts_config: AccountsConfig = if file_path.exists() {
        load_json(&file_path)?
    } else {
        // Si el archivo no existe, no hay nada que hacer.
        return Ok(());
    };

    // 2. Usar `retain` para mantener solo las cuentas cuyo ID no coincide con el que se quiere eliminar.
    // `retain` es eficiente ya que modifica el vector en el lugar sin crear uno nuevo.
    accounts_config.accounts.retain(|account| account.id != account_id_to_remove);

    // 3. Guardar la lista (posiblemente) modificada de nuevo en el archivo.
    save_json(file_path, &accounts_config)?;

    Ok(())
}