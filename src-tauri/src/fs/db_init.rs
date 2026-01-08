use rusqlite::{Connection, Result};
use std::path::Path;

pub fn init_sqlite(conn: &mut Connection) -> Result<()> {
    // Activamos llaves foráneas
    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    // Para los PRAGMA que devuelven resultados (como journal_mode),
    // usamos query_row e ignoramos el resultado.
    let _: String = conn.query_row("PRAGMA journal_mode = WAL;", [], |row| row.get(0))?;

    // Tabla de Cuentas
    conn.execute(
        "CREATE TABLE IF NOT EXISTS workspaces (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL,
        base_path TEXT NOT NULL,
        created_at INTEGER NOT NULL
        );
        ",
        [],
    )?;

    // Tabla de Categorías
    conn.execute(
        "CREATE TABLE IF NOT EXISTS categories (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            type TEXT NOT NULL, -- 'expense' o 'income' o 'transfer'
            icon TEXT,
            color TEXT,
            is_active INTEGER DEFAULT 1
        )",
        [],
    )?;

    // Tabla de Transacciones (Records)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS accounts (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            type TEXT NOT NULL,
            currency TEXT NOT NULL,
            initial_balance REAL NOT NULL,
            is_active INTEGER NOT NULL,
            created_at INTEGER NOT NULL
            );
        ",
        [],
    )?;

    conn.execute(
        "
    CREATE TABLE IF NOT EXISTS tags (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    created_at INTEGER NOT NULL
    );
    ",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS records (
        id TEXT PRIMARY KEY,
        type TEXT NOT NULL,
        timestamp INTEGER NOT NULL,
        amount REAL NOT NULL,
        account_id TEXT NOT NULL,
        to_account_id TEXT NULL,
        category_id TEXT,
        description TEXT,
        file_path TEXT NOT NULL
        );
    ",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS record_tags (
  record_id TEXT NOT NULL,
  tag_id TEXT NOT NULL,
  PRIMARY KEY (record_id, tag_id)
);
    ",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS sync_state (
        file_path TEXT PRIMARY KEY,
        last_modified INTEGER NOT NULL,
        hash TEXT NOT NULL
        );
    ",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS meta (
        key TEXT PRIMARY KEY,
        value TEXT NOT NULL
        );
    ",
        [],
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO meta (key, value) VALUES ('schema_version', '1')",
        [],
    )?;

    Ok(())
}
