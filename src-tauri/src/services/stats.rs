/// Statistics calculation service

use crate::domain::error::AppError;
use rusqlite::{params, Connection, Result};
use serde::Serialize;

#[derive(Serialize)]
pub struct DashboardStats {
    pub total_balance: f64,
    pub total_income: f64,
    pub total_expense: f64,
}

#[derive(Serialize)]
pub struct CategoryPercentage {
    pub category_name: String,
    pub color: String,
    pub amount: f64,
    pub percentage: f64,
}

pub fn calculate_overall_stats(conn: &mut Connection) -> Result<DashboardStats, AppError> {
    // 1. Obtener la suma de todos los balances iniciales
    let initial_balances_sum: f64 = conn.query_row(
        "SELECT SUM(initial_balance) FROM accounts",
        [],
        |row| row.get(0)
    ).unwrap_or(0.0);

    // 2. Obtener la suma de ingresos y gastos
    let mut stmt = conn
        .prepare(
            "SELECT 
                SUM(CASE WHEN type = 'income' THEN amount ELSE 0 END) as income,
                SUM(CASE WHEN type = 'expense' THEN amount ELSE 0 END) as expense
             FROM records",
        )
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let stats = stmt
        .query_row([], |row| {
            let income: f64 = row.get(0).unwrap_or(0.0);
            let expense: f64 = row.get(1).unwrap_or(0.0);
            
            // 3. El balance real es: Saldo Inicial + Ingresos - Gastos
            Ok(DashboardStats {
                total_income: income,
                total_expense: expense,
                total_balance: initial_balances_sum + income - expense,
            })
        })
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(stats)
}

pub fn get_expenses_by_category(
    conn: &Connection,
    days: i64,
) -> Result<Vec<CategoryPercentage>, AppError> {
    let seconds_ago = days * 24 * 60 * 60;

    let mut stmt = conn.prepare(
        "SELECT 
            c.name, c.color, SUM(r.amount) as total,
            (SUM(r.amount) * 100.0 / (SELECT SUM(amount) FROM records WHERE type = 'expense' AND timestamp > strftime('%s', 'now') - ?1)) as percent
         FROM records r
         JOIN categories c ON r.category_id = c.id
         WHERE r.type = 'expense' AND r.timestamp > strftime('%s', 'now') - ?1
         GROUP BY c.id"
    ).map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let rows = stmt
        .query_map(params![seconds_ago], |row| {
            Ok(CategoryPercentage {
                category_name: row.get(0)?,
                color: row.get(1)?,
                amount: row.get(2)?,
                percentage: row.get(3)?,
            })
        })
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row.map_err(|e| AppError::DatabaseError(e.to_string()))?);
    }
    Ok(results)
}
