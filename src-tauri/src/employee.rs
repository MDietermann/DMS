use rusqlite::{params, Connection, Result};
use crate::{custom_errors::{self, CommandResult, CustomRusqliteErrorType}, database_handler};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Employee {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
    position: String,
    password: String
}

impl Employee {
    pub fn check_password(&self, password: &str) -> bool {
        self.password == password
    }

    pub fn invalid_user() -> Employee {
        Employee {
            id: -1,
            first_name: String::new(),
            last_name: String::new(),
            email: String::new(),
            position: String::new(),
            password: String::new(),
        }
    }

    pub async fn get_employee_by_id(employee_id: i32) -> Result<Employee> {
        let connection = Connection::open("dms.db")?;
        let mut query = connection.prepare("SELECT * FROM employee WHERE id = ?1;")?;
        let mut person_iter = query.query_map(params![employee_id], |row| {
            Ok(Employee {
                id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                email: row.get(3)?,
                position: row.get(4)?,
                password: row.get(5)?
            })
        })?;

        let person = person_iter.next().unwrap();
        return person;
    }

    pub async fn add_employee(&self) -> CommandResult<(), CustomRusqliteErrorType> {
        let connection = Connection::open("dms.db")?;

        let mut query = connection.prepare("SELECT * FROM employee WHERE email = ?1;")?;
        let mut person_iter = query.query_map(params![self.email], |row| {
            Ok(Employee {
                id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                email: row.get(3)?,
                position: row.get(4)?,
                password: row.get(5)?
            })
        })?;

        if let Some(next) = person_iter.next() {
            return Err(custom_errors::get_custom_rusqlite_errors(Some(next.unwrap_err())));
        }

        connection.execute(
            "INSERT INTO employee (first_name, last_name, email, position, password)
            VALUES (?1, ?2, ?3, ?4, ?5)",
            params![self.first_name, self.last_name, self.email, self.position, self.password]
        )?;

        Ok(())
    }

    pub async fn get_all_employees() -> Result<Vec<Employee>> {
        let conn = database_handler::get_connection().await?;
        let mut stmt = conn.prepare("SELECT * FROM employee;")?;

        let mut employees = Vec::new();
        let mut employee_iter = stmt.query([])?;
        while let Some(row) = employee_iter.next()? {
            employees.push(Employee {
                id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                email: row.get(3)?,
                position: row.get(4)?,
                password: row.get(5)?,
            });
        }

        Ok(employees)
    }
}
