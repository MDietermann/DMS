#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[path ="database_handler/mod.rs"]
mod database_handler;

mod tauri_commands;
mod ip_factory;
mod type_caster;
mod employee;

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
            tauri_commands::add_employee
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
