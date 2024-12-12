use rusqlite::{params, Connection, Row};
use crate::custom_errors::{self, CommandResult, CustomRusqliteErrorType};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Employee {
    id: Option<i32>,
    first_name: String,
    last_name: String,
    email: String,
    position: String,
    password: String,
    is_admin: bool
}

impl Employee {
    pub fn new(id: Option<i32>, first_name: String, last_name: String, email: String, position: String, password: String, is_admin: bool) -> Employee {
        Employee {
            id: id,
            first_name,
            last_name,
            email,
            position,
            password,
            is_admin
        }
    }

    fn clone(&self) -> Self {
        // Create a new Employee instance and copy the fields from self to it
        Employee {
            id: self.id,
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
            position: self.position.clone(),
            password: self.password.clone(),
            is_admin: self.is_admin
        }
    }

    pub fn create_table(&self, conn: &Connection) -> CommandResult<(), CustomRusqliteErrorType> {
        let db_connection = conn;
        db_connection.execute(
            "CREATE TABLE IF NOT EXISTS employee (
                id INTEGER PRIMARY KEY,
                first_name TEXT NOT NULL,
                last_name TEXT NOT NULL,
                email TEXT NOT NULL,
                position TEXT NOT NULL,
                password TEXT NOT NULL,
                is_admin BOOLEAN
            )",
            params![],
        )?;

        db_connection.execute(
            "INSERT INTO employee (first_name, last_name, email, position, password, is_admin)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![self.first_name, self.last_name, self.email, self.position, self.password, true],
        )?;

        Ok(())
    }

    pub fn check_password(&self, password: &str) -> bool {
        self.password == password
    }

    pub fn invalid_user() -> Employee {
        Employee {
            id: None,
            first_name: String::new(),
            last_name: String::new(),
            email: String::new(),
            position: String::new(),
            password: String::new(),
            is_admin: false
        }
    }

    pub async fn get_employee_by_id(employee_id: i32, conn: &Connection) -> CommandResult<Employee, CustomRusqliteErrorType> {
        let db_connection = conn;

        let mut query = db_connection.prepare("SELECT * FROM employee WHERE id = ?1;")?;
        let mut employee_iter = query.query_map([employee_id], |row| {
            Ok(Employee {
                id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                email: row.get(3)?,
                position: row.get(4)?,
                password: row.get(5)?,
                is_admin: row.get(6)?
            })
        })?;

        let employee = match employee_iter.next().transpose()? {
            Some(employee) => employee,
            None => return Err(custom_errors::get_custom_rusqlite_errors(None)),
        };

        Ok(employee)
    }

    pub async fn add_employee(&self, conn: &Connection) -> CommandResult<(), CustomRusqliteErrorType> {
        let mut query = conn.prepare("SELECT * FROM employee WHERE email = ?1;")?;
        let mut person_iter = query.query_map(params![self.email], |row| {
            Ok(Employee {
                id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                email: row.get(3)?,
                position: row.get(4)?,
                password: row.get(5)?,
                is_admin: row.get(6)?
            })
        })?;

        if let Some(next) = person_iter.next() {
            return Err(custom_errors::get_custom_rusqlite_errors(Some(next.unwrap_err())));
        }

        conn.execute(
            "INSERT INTO employee (first_name, last_name, email, position, password)
            VALUES (?1, ?2, ?3, ?4, ?5)",
            params![self.first_name, self.last_name, self.email, self.position, self.password]
        )?;

        Ok(())
    }

    pub fn from_row(row: &Row) -> rusqlite::Result<Employee> {
        Ok(Employee {
            id: row.get(0)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            email: row.get(3)?,
            position: row.get(4)?,
            password: row.get(5)?,
            is_admin: row.get(6)?
        })
    }
}
