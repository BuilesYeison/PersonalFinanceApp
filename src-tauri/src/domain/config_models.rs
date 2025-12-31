use crate::{commands::workspace, helpers::datetime_helpers::timestamp_now};
use serde::{Deserialize, Serialize};

// --- Version ---
#[derive(Serialize, Deserialize, Debug)]
pub struct VersionConfig {
    pub schema_version: u32,
    pub created_at: u64,
    pub app_version: String,
}

impl Default for VersionConfig {
    fn default() -> Self {
        Self {
            schema_version: 1,
            created_at: timestamp_now(),
            app_version: "0.1.0".to_string(),
        }
    }
}

// --- App Global ---
#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub currency: String,
    pub language: String,
    pub theme: String,
    pub week_start: String,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        let t = timestamp_now();
        Self {
            currency: "COP".into(),
            language: "es".into(),
            theme: "system".into(),
            week_start: "monday".into(),
            created_at: t,
            updated_at: t,
        }
    }
}

// --- Categories ---
#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryItem {
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub icon: String,
    pub color: String,
    pub created_by_user: bool,
    pub is_active: bool,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoriesConfig {
    pub categories: Vec<CategoryItem>,
}

impl Default for CategoriesConfig {
    fn default() -> Self {
        let t = timestamp_now();
        let mut categories = Vec::new();

        // Estructura temporal para definir valores por defecto precisos
        let expenses = vec![
            ("cat_food", "Comida", "utensils", "#ef4444"), // Red 500
            ("cat_transport", "Transporte", "car", "#f59e0b"), // Amber 500
            ("cat_health", "Salud", "heart-pulse", "#ec4899"), // Pink 500
            ("cat_edu", "Educación", "graduation-cap", "#8b5cf6"), // Violet 500
            ("cat_auto", "Automóvil", "wrench", "#64748b"), // Slate 500
            ("cat_home", "Vivienda", "home", "#06b6d4"),   // Cyan 500
            ("cat_sports", "Deportes", "dumbbell", "#10b981"), // Emerald 500
            ("cat_entert", "Entretenimiento", "clapperboard", "#f43f5e"), // Rose 500
            ("cat_pets", "Mascotas", "dog", "#d946ef"),    // Fuchsia 500
            ("cat_gifts", "Regalos", "gift", "#fb923c"),   // Orange 400
            ("cat_clothes", "Ropa", "shirt", "#6366f1"),   // Indigo 500
            ("cat_services", "Servicios", "zap", "#eab308"), // Yellow 500
            ("cat_taxes", "Impuestos", "receipt", "#475569"), // Slate 600
        ];

        for (id, name, icon, color) in expenses {
            categories.push(CategoryItem {
                id: id.into(),
                name: name.into(),
                r#type: "expense".into(),
                icon: icon.into(),
                color: color.into(),
                created_by_user: false,
                is_active: true,
                created_at: t,
            });
        }

        let incomes = vec![
            ("cat_salary", "Salario", "banknote", "#22c55e"), // Green 500
            ("cat_extra", "Ingresos extra", "trending-up", "#34d399"), // Emerald 400
            ("cat_others", "Otros ingresos", "wallet", "#2dd4bf"), // Teal 400
        ];

        for (id, name, icon, color) in incomes {
            categories.push(CategoryItem {
                id: id.into(),
                name: name.into(),
                r#type: "income".into(),
                icon: icon.into(),
                color: color.into(),
                created_by_user: false,
                is_active: true,
                created_at: t,
            });
        }

        Self { categories }
    }
}

// --- Accounts ---
#[derive(Serialize, Deserialize, Debug)]
pub struct AccountItem {
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub currency: String,
    pub initial_balance: f64,
    pub credit_limit: Option<f64>, // Puede ser null
    pub is_active: bool,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountsConfig {
    pub accounts: Vec<AccountItem>,
}

impl Default for AccountsConfig {
    fn default() -> Self {
        let t = timestamp_now();
        Self {
            accounts: vec![AccountItem {
                id: "acc_cash".into(),
                name: "Efectivo".into(),
                r#type: "cash".into(),
                currency: "COP".into(),
                initial_balance: 0.0,
                credit_limit: None,
                is_active: true,
                created_at: t,
            }],
        }
    }
}

// --- Last session ---
#[derive(Serialize, Deserialize, Debug)]
pub struct LastSessionCacheConfig {
    pub last_workspace_path: String,
    pub last_workspace_name: String,
}
