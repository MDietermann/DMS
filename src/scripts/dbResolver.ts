export const resolveDbName = (rawName: String) => {
    switch (rawName) {
        case "mysql": return "MySQL"
        case "postgresql": return "PostgreSQL"
        case "mariadb": return "MariaDB"
        case "sqlserver": return "SQL-Server"
        default: return `Resolve not found for: ${rawName}`
    }
}

export const resolveUserDataNames = (rawName: String) => {
    switch (rawName) {
        case "id": return "Mitarbeiter-ID"
        case "first_name": return "Vorname"
        case "last_name": return "Nachname"
        case "email": return "E-Mail"
        case "position": return "Position"
        default: return `Resolve not found for: ${rawName}`
    }
}
