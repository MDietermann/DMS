use crate::custom_errors::{ CustomRusqliteError, CommandResult };
use crate::enums::export_types::ExportType;
use crate::sqlite_handler::{ Employee, SqliteFactory, ResultType, DbController, TsStructure };

#[tauri::command]
pub async fn login(employee_id: i32, passwd: String) -> CommandResult<Employee, CustomRusqliteError> {
    let factory = SqliteFactory::new("employee".to_string(), None);
    let result = factory.get_by_id(employee_id).await?;
    let first_result = result.first().ok_or(CustomRusqliteError::InvalidCredentials("User not found".to_string()))?;
    let user: Employee = match first_result {
        ResultType::Employee(e) => (*e).clone(),
        _ => return Err(CustomRusqliteError::InvalidCredentials("Invalid user type".to_string())),
    };
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
pub fn create_database() -> CommandResult<(), CustomRusqliteError> {
    let db_path = "dms.db";
    let controller = DbController::new(db_path).expect("Failed to create database controller");

    match controller.create_tables() {
        Ok(_) => Ok(()),
        Err(e) => Err(CustomRusqliteError::DatabaseError(format!("Failed to create database: {}", e))),
    }
}

#[tauri::command]
pub fn get_database_data() -> CommandResult<Vec<TsStructure>, CustomRusqliteError> {
    let db_path = "dms.db";
    let controller = DbController::new(db_path).expect("Failed to create database controller");

    match controller.generate_ts_structure() {
        Ok(structure) => Ok(structure),
        Err(e) => Err(CustomRusqliteError::DatabaseError(format!("Failed to generate TS structure: {}", e))),
    }
}

#[tauri::command]
pub fn add_to_database(structure: Vec<TsStructure>) -> CommandResult<(), CustomRusqliteError> {
    let db_path = "dms.db";
    let controller = DbController::new(db_path).expect("Failed to create database controller");

    match controller.import_ts_structure(structure) {
        Ok(_) => Ok(()),
        Err(e) => Err(CustomRusqliteError::DatabaseError(format!("Failed to import TS structure: {}", e))),
    }
}

#[tauri::command]
async fn export_database_table(
    database_type: String,
    server_ip: String,
    username: String,
    table: String,
    export_type: ExportType
    ) -> Result<(), String> {
  Ok(())
}

// #[tauri::command]
// pub async fn add_employee(employee: Employee) -> CommandResult<(), CustomRusqliteError> {
//     let factory = SqliteFactory::new("employee".to_string(), None);
//     let conn = match factory.get_connection().await {
//         Ok(conn) => conn,
//         Err(e) => return Err(CustomRusqliteError::DatabaseError(format!("Failed to get connection: {}", e))),
//     };
//     match employee.add_employee().await {
//         Ok(_) => Ok(()),
//         Err(e) => Err(CustomRusqliteError::DatabaseError(format!("Failed to add employee: {}", e))),
//     }
// }
