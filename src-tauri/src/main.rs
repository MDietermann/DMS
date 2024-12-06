#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use sqlite_handler::Employee;
mod sqlite_handler;

// Our Tauri Command
#[tauri::command]
async fn login(employee_id: i32, passwd: String) -> Employee {
    let user = Employee::get_employee_by_id(employee_id).await.unwrap();
    if user.check_password(passwd) {
        return user;
    } else {
        return Employee::get_invalid_user();
    }
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        // Register Command with Tauri App
        .invoke_handler(tauri::generate_handler![login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
        let _result = sqlite_handler::create().await;
}
