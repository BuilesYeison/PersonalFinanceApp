mod commands;
mod domain;
mod dto;
mod fs;
mod helpers;
mod services;

struct AppState {
    // Usamos un Mutex porque la DB se comparte entre hilos
    db: std::sync::Mutex<Option<rusqlite::Connection>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            db: std::sync::Mutex::new(None),
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::workspace::init_workspace,
            commands::workspace::open_workspace,
            commands::workspace::get_workspace_context,
            commands::home::get_overall_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
