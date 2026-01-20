// Accounts functionality

use crate::domain::error::AppError;
use crate::dto::account_info_dto::AccountInfoDto;
use rusqlite::{Connection, Result};

pub fn get_accounts_with_balance(conn: &mut Connection) -> Result<Vec<AccountInfoDto>, AppError> {
    let mut stmt = conn
        .prepare(
            r#"
            SELECT 
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
            let initial_balance: f64 = row.get(1)?;
            let movement_sum: Option<f64> = row.get(2)?;

            Ok(AccountInfoDto {
                name: row.get(0)?,
                balance: initial_balance + movement_sum.unwrap_or(0.0),
                account_type: None,
                currency: None,
                initial_balance:None,
                credit_limit:None,
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
