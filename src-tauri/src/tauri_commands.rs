use crate::custom_errors::{ CustomRusqliteError, CommandResult };
use crate::sqlite_handler::{ Employee, SqliteFactory, ResultType };

#[tauri::command]
pub async fn login(employee_id: i32, passwd: String) -> CommandResult<Employee, CustomRusqliteError> {
    let user = Employee::get_employee_by_id(employee_id).await?;
    if user.check_password(&passwd) {
        Ok(user)
    } else {
        Err(CustomRusqliteError::InvalidCredentials("Incorrect password".to_string()))
    }
}

#[tauri::command]
pub async fn get_all_employees() -> CommandResult<Vec<Employee>, CustomRusqliteError> {
    let factory = SqliteFactory::new("employee".to_string(), None);
    match factory.get_all().await {
        Ok(employees) => {
            let employees: Vec<Employee> = employees
                .into_iter()
                .map(|result_type| match result_type {
                    ResultType::Employee(employee) => employee,
                    _ => {
                        // You might log or handle invalid rows more specifically here
                        Employee::invalid_user()
                    }
                })
                .collect();
            Ok(employees)
        },
        Err(e) => Err(CustomRusqliteError::DatabaseError(format!("Failed to fetch employees: {}", e))),
    }
}


#[tauri::command]
pub async fn add_employee(employee: Employee) -> CommandResult<(), CustomRusqliteError> {
    match employee.add_employee().await {
        Ok(_) => Ok(()),
        Err(e) => Err(CustomRusqliteError::DatabaseError(format!("Failed to add employee: {}", e))),
    }
}
