// Accounts functionality

use crate::dto::account_info_dto::AccountInfoDto;
use crate::{domain::error::AppError, helpers::datetime_helpers::timestamp_now};
use rusqlite::{Connection, Result};

pub fn get_accounts_with_balance(conn: &mut Connection) -> Result<Vec<AccountInfoDto>, AppError> {
    let mut stmt = conn
        .prepare(
            r#"
            SELECT 
                a.id,
                a.name,
                a.initial_balance,
                SUM(
                    CASE
                        WHEN r.type = 'income' THEN r.amount
                        WHEN r.type = 'expense' THEN -r.amount
                        ELSE 0
                    END
                ) as balance
            FROM accounts a
            LEFT JOIN records r ON r.account_id = a.id
            GROUP BY a.id, a.name, a.initial_balance
            "#,
        )
        .map_err(|e| {
            AppError::DatabaseError(format!("Error preparando get_accounts_with_balance: {e}"))
        })?;

    let rows = stmt
        .query_map([], |row| {
            let initial_balance: f64 = row.get(2)?;
            let movement_sum: Option<f64> = row.get(3)?;

            Ok(AccountInfoDto {
                id: row.get(0)?,
                name: row.get(1)?,
                balance: initial_balance + movement_sum.unwrap_or(0.0),
                account_type: None,
                currency: None,
                initial_balance: None,
                credit_limit: None,
            })
        })
        .map_err(|e| {
            AppError::DatabaseError(format!("Error ejecutando get_accounts_with_balance: {e}"))
        })?;

    let mut accounts = Vec::new();
    for row in rows {
        accounts.push(row.map_err(|e| {
            AppError::DatabaseError(format!("Error leyendo fila get_accounts_with_balance: {e}"))
        })?);
    }

    Ok(accounts)
}

pub fn create_account_in_database(
    conn: &mut Connection,
    account: AccountInfoDto,
    account_id: &str,
) -> Result<(), AppError> {
    let created_at = timestamp_now();
    let account_type = account.account_type.unwrap_or_else(|| "cash".to_string());
    let currency = account.currency.unwrap_or_else(|| "USD".to_string());
    let initial_balance = account.initial_balance.unwrap_or(0.0);

    conn.execute(
        "INSERT INTO accounts (id, name, type, currency, initial_balance, is_active, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (
            &account_id,
            &account.name,
            &account_type,
            &currency,
            initial_balance,
            1, // is_active
            created_at,
        ),
    )
    .map_err(|e| AppError::DatabaseError(format!("Error creando cuenta: {e}")))?;

    Ok(())
}

pub fn delete_account_if_no_records(
    conn: &mut Connection,
    account_id: &str,
) -> Result<bool, AppError> {
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM records WHERE account_id = ?1",
            [account_id],
            |row| row.get(0),
        )
        .map_err(|e| {
            AppError::DatabaseError(format!("Error verificando registros de la cuenta: {e}"))
        })?;

    if count > 0 {
        return Ok(false);
    }

    conn.execute("DELETE FROM accounts WHERE id = ?1", [account_id])
        .map_err(|e| AppError::DatabaseError(format!("Error eliminando la cuenta: {e}")))?;

    Ok(true)
}
