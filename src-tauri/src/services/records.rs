use crate::domain::error::AppError;
use crate::dto::category_dto::CategoryDto;
use crate::dto::pagination_dto::Pagination;
use crate::dto::{account_info_dto::AccountInfoDto, record_dto::RecordDto};
use rusqlite::{params, Connection};

pub fn get_records(
    conn: &mut Connection,
    page: i16,
    size: i16,
) -> Result<Pagination<RecordDto>, AppError> {
    let offset: i64 = (page as i64 - 1) * size as i64;

    // 1. QUERY COMPLEJA: Hacemos JOIN con Accounts (Origen), Accounts (Destino) y Categories
    // Usamos alias: 'a' para cuenta origen, 'ta' para to_account, 'c' para category
    let sql = "
        SELECT 
            r.id, r.type, r.timestamp, r.amount, r.description,
            -- Cuenta Origen (Indices 5-9)
            a.id, a.name, a.type, a.currency, a.initial_balance,
            -- Cuenta Destino (Indices 10-14) - Puede ser NULL
            ta.id, ta.name, ta.type, ta.currency, ta.initial_balance,
            -- Categoría (Indices 15-20) - Puede ser NULL
            c.id, c.name, c.type, c.icon, c.color, c.is_active
        FROM records r
        LEFT JOIN accounts a ON r.account_id = a.id
        LEFT JOIN accounts ta ON r.to_account_id = ta.id
        LEFT JOIN categories c ON r.category_id = c.id
        ORDER BY r.timestamp DESC
        LIMIT ?1 OFFSET ?2
    ";

    let mut stmt = conn
        .prepare(sql)
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let rows = stmt
        .query_map(params![size, offset], |row| {
            // --- 1. Construir Cuenta Origen (AccountInfoDto) ---
            // Asumimos que account_id siempre existe (NOT NULL en tabla records)
            let acc_currency: String = row.get(8)?;
            let account_dto = AccountInfoDto {
                id: row.get(5)?,
                name: row.get(6)?,
                account_type: row.get(7)?,
                currency: Some(acc_currency.clone()),
                // Nota: En un listado transaccional, 'balance' suele ser el saldo al momento
                // o el actual. Aquí ponemos initial_balance por rendimiento.
                balance: row.get(9)?,
                initial_balance: row.get(9)?,
                credit_limit: None, // No está en tu tabla 'accounts' actual
            };

            // --- 2. Construir Cuenta Destino (Option<AccountInfoDto>) ---
            let to_account_id: Option<String> = row.get(10)?;
            let to_account_dto = if to_account_id.is_some() {
                Some(AccountInfoDto {
                    id: to_account_id.unwrap(),
                    name: row.get(11)?,
                    account_type: row.get(12)?,
                    currency: row.get(13)?,
                    balance: row.get(14)?, // Simplificación
                    initial_balance: row.get(14)?,
                    credit_limit: None,
                })
            } else {
                None
            };

            // --- 3. Construir Categoría (Option<CategoryDto>) ---
            let category_id: Option<String> = row.get(15)?;
            let category_dto = if category_id.is_some() {
                Some(CategoryDto {
                    id: category_id.unwrap(),
                    name: row.get(16)?,
                    r#type: row.get(17)?,
                    icon: row.get(18)?,
                    color: row.get(19)?,
                    is_active: row.get::<_, i32>(20)? == 1, // Convertir Integer sqlite a Bool
                })
            } else {
                None
            };

            // --- 4. Construir RecordDto Final ---
            Ok(RecordDto {
                id: row.get(0)?,
                r#type: row.get(1)?,
                timestamp: row.get(2)?,
                amount: row.get(3)?,
                description: row.get(4)?,
                currency: Some(acc_currency), // Heredamos la moneda de la cuenta origen
                account: Some(account_dto),
                to_account: to_account_dto,
                category: category_dto,
            })
        })
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Recolectar resultados
    let mut items = Vec::new();
    for row in rows {
        items.push(row.map_err(|e| AppError::DatabaseError(e.to_string()))?);
    }

    // 2. Obtener total de items para paginación
    let total_items: i64 = conn
        .query_row("SELECT COUNT(*) FROM records", [], |row| row.get(0))
        .unwrap_or(0);

    let total_pages = if size > 0 {
        (total_items as f64 / size as f64).ceil() as i64
    } else {
        0
    };

    Ok(Pagination {
        items,
        total_items,
        current_page: page,
        size,
        total_pages,
    })
}
