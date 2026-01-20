use crate::domain::error::AppError;
use crate::helpers::json_helpers::{load_json, save_json};
use uuid::Uuid;
use std::path::PathBuf;
use crate::domain::config_models::{AccountItem,AccountsConfig};
use crate::dto::account_info_dto::AccountInfoDto;
use crate::helpers::datetime_helpers::timestamp_now;


// Función para agregar una cuenta a la lista
pub fn add_account_to_list(file_path: PathBuf, new_account: AccountInfoDto) -> Result<(), AppError> {
    // 2. Cargar la lista existente
    // Verificamos si el archivo existe primero para evitar errores de IO si es la primera vez
    let mut accounts_config: AccountsConfig = if file_path.exists() {
        load_json(&file_path)?
    } else {
        AccountsConfig::default()
    };

    let new_account_item = AccountItem {
        id: Uuid::new_v4().to_string(),
        name: new_account.name.clone(),
        r#type: new_account.account_type.unwrap_or_default().clone(),
        currency: new_account.currency.unwrap_or_default().clone(), // Usamos la moneda del workspace
        initial_balance: new_account.initial_balance.unwrap_or_default().clone(),
        credit_limit: None, // El formulario básico no lo pide, se deja en None
        is_active: true,
        created_at: timestamp_now(),
    };

    // 3. Agregar el nuevo objeto a la lista en memoria
    accounts_config.accounts.push(new_account_item);

    // 4. Guardar la lista actualizada en el archivo
    save_json(file_path, &accounts_config)?;

    Ok(())
}
