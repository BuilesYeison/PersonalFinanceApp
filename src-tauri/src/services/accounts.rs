// Accounts functionality

use crate::domain::error::AppError;
use rusqlite::{Connection, Result};
use serde::Serialize;

#[derive(Serialize)]
pub struct AccountInfoDto {
    pub name: String,
    pub balance: f64,
}

pub fn get_accounts_with_balance(conn: &mut Connection) -> Result<Vec<AccountInfoDto>, AppError> {
    let mut stmt = conn
        .prepare(
            "SELECT 
            a.name,
            a.initial_balance,
            SUM(
                CASE
                WHEN type = 'income' THEN amount
                WHEN type = 'expense' THEN -amount
                ELSE 0
                END
            ) as balance
            FROM records r
            JOIN accounts a ON r.account_id = a.id
            GROUP BY a.name",
        )
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let rows = stmt.query_map([], |row| {
        {
            Ok(AccountInfoDto {
                name: row.get(0)?,
                balance: row.get::<_, f64>(1)? + row.get::<_, f64>(2)?,
            })
        }    
    }).map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let mut accounts = Vec::new();
    for row in rows {
        accounts.push(row.map_err(|e| AppError::DatabaseError(e.to_string()))?);
    }

    Ok(accounts)
}
