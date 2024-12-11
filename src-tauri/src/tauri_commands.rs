use crate::custom_errors::{ CustomRusqliteErrorType, CommandResult, self };
use crate::sqlite_handler::{ Employee, SqliteFactory, ResultType };

#[tauri::command]
pub async fn login(employee_id: i32, passwd: String) -> CommandResult<Employee, CustomRusqliteErrorType> {
    let user = Employee::get_employee_by_id(employee_id).await?;
    if user.check_password(&passwd) {
        Ok(user)
    } else {
        Err(custom_errors::get_custom_rusqlite_errors(None))
    }
}

#[tauri::command]
pub async fn get_all_employees() -> CommandResult<Vec<Employee>, CustomRusqliteErrorType> {
    let factory = SqliteFactory::new("employee".to_string());
    match factory.get_all().await {
        Ok(employees) => {
            let employees: Vec<Employee> = employees
                .into_iter()
                .map(|result_type| match result_type {
                    ResultType::Employee(employee) => employee,
                    _ => Employee::invalid_user(),
                })
                .collect();
            Ok(employees)
        },
        Err(_) => Err(custom_errors::get_custom_rusqlite_errors(None)),
    }
}

#[tauri::command]
pub async fn add_employee(employee: Employee) -> CommandResult<(), CustomRusqliteErrorType> {
    match employee.add_employee().await {
        Ok(_) => Ok(()),
        Err(_) => Err(custom_errors::get_custom_rusqlite_errors(None)),
    }
}
