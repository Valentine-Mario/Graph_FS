pub const CREATE_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    permission TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);"#;
