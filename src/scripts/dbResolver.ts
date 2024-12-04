export const resolveDbName = (rawName: String) => {
    switch (rawName) {
        case "mysql": return "MySQL"
        case "postgresql": return "PostgreSQL"
        case "mariadb": return "MariaDB"
        case "sqlserver": return "SQL-Server"
        default: return `Resolve not found for: ${rawName}`
    }
}
