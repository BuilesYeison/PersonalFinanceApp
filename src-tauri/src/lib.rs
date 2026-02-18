mod commands;
mod domain;
mod dto;
mod fs;
mod helpers;
mod services;

pub struct AppState {
    // Conexión a la base de datos
    pub db: std::sync::Mutex<Option<rusqlite::Connection>>,
    // Ruta absoluta al workspace actual
    pub workspace_path: std::sync::Mutex<Option<std::path::PathBuf>>,
    // app_data_dir
    pub workspace_app_data_dir: std::sync::Mutex<Option<std::path::PathBuf>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            db: std::sync::Mutex::new(None),
            workspace_path: std::sync::Mutex::new(None),
            workspace_app_data_dir: std::sync::Mutex::new(None),
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::workspace::init_workspace,
            commands::workspace::open_workspace,
            commands::workspace::get_workspace_context,
            commands::home::get_overall_stats,
            commands::home::get_accounts,
            commands::home::create_account,
            commands::home::update_account,
            commands::home::delete_account,
            commands::home::get_paginated_records,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
