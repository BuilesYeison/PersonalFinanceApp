use crate::domain::config_models::RecordItem;
use crate::domain::error::AppError;
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Serialize)]
pub struct Pagination<T> {
    items: Vec<T>,
    total_items: i64,
    current_page: i16,
    size: i16,
    total_pages: i64,
}

pub fn get_records(
    conn: &mut Connection,
    page: i16,
    size: i16,
) -> Result<Pagination<RecordItem>, AppError> {
    let offset: i64 = (page as i64 - 1) * size as i64;
    let mut stmt = conn.prepare("SELECT id, type, timestamp, amount, account_id, to_account_id, category_id, description FROM records ORDER BY timestamp DESC LIMIT ? OFFSET ?")
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let mut total_rows = 0;
    let rows = stmt
        .query_map(params![size, offset], |row| {
            // total_rows = row.get(9)?; // This line is problematic as it tries to assign to total_rows inside the closure
            Ok(RecordItem {
                id: row.get(0)?,
                r#type: row.get(1)?,
                timestamp: row.get(2)?,
                amount: row.get(3)?,
                account_id: row.get(4)?,
                to_account_id: row.get(5)?,
                category_id: row.get(6)?,
                description: row.get(7)?,
                metadata: None,
                currency: None,
            })
        })
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row.map_err(|e| AppError::DatabaseError(e.to_string()))?);
    }
    
    // Get total_rows separately
    let mut count_stmt = conn.prepare("SELECT COUNT(*) FROM records")
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    total_rows = count_stmt.query_row([], |row| row.get(0)).map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(Pagination {
        items: results,
        total_items: total_rows,
        current_page: page,
        size,
        total_pages: (total_rows as f64 / size as f64).ceil() as i64,
    })
}
