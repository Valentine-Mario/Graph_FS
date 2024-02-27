pub const CREATE_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    permission TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);"#;

pub const GET_USER_BY_EMAIL_SQL: &str = r#"
    SELECT id, name, email, password, permission
    FROM users
    WHERE email = $1
"#;

pub const GET_USER_BY_ID_SQL: &str = r#"
    SELECT id, name, email, password, permission
    FROM users
    WHERE id = $1
"#;

pub const DELETE_USER_SQL: &str = r#"
    DELETE from users
    WHERE email = $1
"#;

pub const CRAETE_NEW_USER_SQL: &str = r#"
    INSERT INTO users ( name, email, password,  permission)
    VALUES ( $1, $2, $3, $4 )
    RETURNING id
"#;

pub const UPDATE_PERMISSION_SQL: &str = r#"
    UPDATE users
    SET permission = $1
    WHERE email = $2
"#;

pub const UPDATE_PASSWORD_SQL: &str = r#"
    UPDATE users
    SET password = $1
    WHERE email = $2
"#;

pub const UPDATE_EMAIL_SQL: &str = r#"
    UPDATE users
    SET email = $1
    WHERE email = $2
"#;

pub const UPDATE_NAME_SQL: &str = r#"
    UPDATE users
    SET name = $1
    WHERE email = $2
"#;
