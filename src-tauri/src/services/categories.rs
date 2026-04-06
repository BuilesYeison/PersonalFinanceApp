use crate::domain::error::AppError;
use crate::dto::category_dto::CategoryDto;
use rusqlite::Connection;

pub fn get_categories(conn: &mut Connection) -> Result<Vec<CategoryDto>, AppError> {
    let mut stmt = conn
        .prepare(
            "SELECT id, name, type, icon, color, is_active FROM categories WHERE is_active = 1",
        )
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let rows = stmt
        .query_map([], |row| {
            Ok(CategoryDto {
                id: row.get(0)?,
                name: row.get(1)?,
                r#type: row.get(2)?,
                icon: row.get(3)?,
                color: row.get(4)?,
                is_active: row.get::<_, i32>(5)? == 1,
            })
        })
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let mut categories = Vec::new();
    for row in rows {
        categories.push(row.map_err(|e| AppError::DatabaseError(e.to_string()))?);
    }

    Ok(categories)
}
