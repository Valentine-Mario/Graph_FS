pub const CREATE_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    permission TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);"#;

pub const GET_USER_SQL: &str = r#"
    SELECT email, password, permission
    FROM users
    WHERE email = $1
"#;

pub const DELETE_USER_SQL: &str = r#"
    DELETE users
    WHERE email = $1
"#;

pub const CRAETE_NEW_USER_SQL: &str = r#"
    INSERT INTO users ( name, email, password,  permission)
    VALUES ( $1, $2, $3, $4 )
    RETURNING id
"#;

pub const UPDATE_PERMISSION_SQL: &str = r#"
    UPDATE users
    SET permission = $2
    WHERE email = $1
"#;

pub const UPDATE_PASSWORD_SQL: &str = r#"
    UPDATE users
    SET password = $2
    WHERE email = $1
"#;

pub const UPDATE_EMAIL_SQL: &str = r#"
    UPDATE users
    SET email = $2
    WHERE email = $1
"#;

pub const UPDATE_NAME_SQL: &str = r#"
    UPDATE users
    SET name = $2
    WHERE email = $1
"#;
