#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


mod tauri_commands;

#[path ="enums/mod.rs"]
mod enums;

#[path ="database_handler/mod.rs"]
mod database_handler;

#[path = "sqlite_handler/mod.rs"]
mod sqlite_handler;

#[path = "custom_errors/mod.rs"]
mod custom_errors;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        // Register Command with Tauri App
        .invoke_handler(tauri::generate_handler![
            tauri_commands::export_data,
            tauri_commands::import_data,
            tauri_commands::test_connection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
