pub const CREATE_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    permission TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);"#;

pub const GET_USER_BY_EMAIL_SQL: &str = r#"
    SELECT id, name, email, password, permission
    FROM users
    WHERE email = ?
"#;

pub const GET_USER_BY_ID_SQL: &str = r#"
    SELECT id, name, email, password, permission
    FROM users
    WHERE id = ?
"#;

pub const DELETE_USER_SQL: &str = r#"
    DELETE users
    WHERE email = ?
"#;

pub const CRAETE_NEW_USER_SQL: &str = r#"
    INSERT INTO users ( name, email, password,  permission)
    VALUES ( ?, ?, ?, ? )
    RETURNING id
"#;

pub const UPDATE_PERMISSION_SQL: &str = r#"
    UPDATE users
    SET permission = ?
    WHERE email = ?
"#;

pub const UPDATE_PASSWORD_SQL: &str = r#"
    UPDATE users
    SET password = ?
    WHERE email = ?
"#;

pub const UPDATE_EMAIL_SQL: &str = r#"
    UPDATE users
    SET email = ?
    WHERE email = ?
"#;

pub const UPDATE_NAME_SQL: &str = r#"
    UPDATE users
    SET name = ?
    WHERE email = ?
"#;
