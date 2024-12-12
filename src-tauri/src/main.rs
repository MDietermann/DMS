#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use sqlite_handler::Employee;

mod tauri_commands;
mod ip_factory;
mod type_caster;

#[path ="database_handler/mod.rs"]
mod database_handler;

#[path = "sqlite_handler/mod.rs"]
mod sqlite_handler;

#[path = "custom_errors/mod.rs"]
mod custom_errors;

#[path ="database_structs/mod.rs"]
mod database_structs;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        // Register Command with Tauri App
        .invoke_handler(tauri::generate_handler![
            tauri_commands::login,
            tauri_commands::get_all_employees,
            tauri_commands::get_database_data,
            tauri_commands::add_to_database,
            tauri_commands::create_database
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
