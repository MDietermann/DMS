export const dbMock: Object = {
    "mysql": [
        { "ip": "192.168.0.1", "user": "root", "tables": ["shop", "shared", "timetable"]},
        { "ip": "192.168.12.1", "user": "developer", "tables": ["shop", "shared"]},
        { "ip": "192.168.43.4", "user": "user1", "tables": ["shared"]}
    ],

    "postgresql": [
        { "ip": "192.168.10.19", "user": "root", "tables": ["shop", "esw", "timetable"]},
        { "ip": "192.168.20.51", "user": "user1", "tables": ["esw"]}
    ],

    "mariadb": [
        { "ip": "192.168.101.51", "user": "root", "tables": ["csw", "hgo", "timetable"]},
        { "ip": "192.168.178.14", "user": "developer", "tables": ["csw", "hgo"]},
        { "ip": "192.168.2.1", "user": "user1", "tables": ["shared"]}
    ],

    "sqlserver": [
        { "ip": "192.168.56.71", "user": "root", "tables": ["test", "sql", "server"]},
        { "ip": "192.168.30.18", "user": "developer", "tables": ["test", "sql"]},
        { "ip": "192.168.46.41", "user": "user1", "tables": ["sql"]}
    ]
}
