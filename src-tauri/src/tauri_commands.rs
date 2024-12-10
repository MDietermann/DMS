use crate::custom_errors::{ CustomRusqliteErrorType, CommandResult, self };
use crate::employee::Employee;

#[tauri::command]
pub async fn login(employee_id: i32, passwd: String) -> Employee {
    let user = Employee::get_employee_by_id(employee_id).await.unwrap();
    if user.check_password(&passwd) {
        println!("password is correct");
        return user;
    } else {
        return Employee::invalid_user();
    }
}

#[tauri::command]
pub async fn get_all_employees() -> CommandResult<Vec<Employee>, CustomRusqliteErrorType> {
    match Employee::get_all_employees().await {
        Ok(employees) => Ok(employees),
        Err(e) => Err(custom_errors::get_custom_rusqlite_errors(Some(e))),
    }
}

#[tauri::command]
pub async fn add_employee(employee: Employee) -> CommandResult<(), CustomRusqliteErrorType> {
    match employee.add_employee().await {
        Ok(_) => Ok(()),
        Err(e) => Err(custom_errors::get_custom_rusqlite_errors(Some(e))),
    }
}
