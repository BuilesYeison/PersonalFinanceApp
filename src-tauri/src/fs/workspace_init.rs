use std::fs;
use std::io::Write;
use std::path::Path;

pub fn init(base_path: String, name: String) -> Result<(), std::io::Error> {
    // 1. Construir ruta del workspace
    let workspace_path = Path::new(&base_path).join(&name);

    // 2. Validar que no exista
    if workspace_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "El workspace ya existe",
        ));
    }

    // 3. Crear carpetas principales
    fs::create_dir_all(&workspace_path)?;
    fs::create_dir(workspace_path.join(".finance"))?;
    fs::create_dir(workspace_path.join("records"))?;
    fs::create_dir(workspace_path.join("attachments"))?;

    // 4. Crear archivos de configuración
    create_file(workspace_path.join(".finance/version.json"), VERSION_JSON)?;

    create_file(workspace_path.join(".finance/app.json"), APP_JSON)?;

    create_file(
        workspace_path.join(".finance/categories.json"),
        CATEGORIES_JSON,
    )?;

    create_file(workspace_path.join(".finance/accounts.json"), ACCOUNTS_JSON)?;

    create_file(workspace_path.join(".finance/budgets.json"), "{}")?;

    create_file(workspace_path.join(".finance/tags.json"), "[]")?;

    Ok(())
}

// -------- helpers --------

fn create_file(path: std::path::PathBuf, content: &str) -> Result<(), std::io::Error> {
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

// -------- templates --------

const VERSION_JSON: &str = r#"
{
  "schema_version": "1.0.0",
  "created_at": 0
}
"#;

const APP_JSON: &str = r#"
{
  "name": "Carpeta Financiera",
  "currency": "COP",
  "language": "es"
}
"#;

const CATEGORIES_JSON: &str = r#"
{
  "expenses": [
    "Comida",
    "Transporte",
    "Salud",
    "Educación",
    "Automóvil",
    "Vivienda",
    "Deportes",
    "Entretenimiento",
    "Mascotas",
    "Regalos",
    "Ropa"
  ],
  "income": [
    "Salario",
    "Depósitos",
    "Ahorros"
  ]
}
"#;

const ACCOUNTS_JSON: &str = r#"
[
  { "name": "Efectivo", "type": "cash" },
  { "name": "Cuenta Débito", "type": "debit" },
  { "name": "Tarjeta Crédito", "type": "credit" }
]
"#;
