use std::env;

use crate::cli::Args;
pub fn args_factory() -> Args {
    Args {
        host: Some("host".to_string()),
        port: Some(12),
        authorized_path: Some(
            env::current_dir()
                .unwrap()
                .as_path()
                .to_str()
                .unwrap()
                .to_string(),
        ),
        worker: Some(2),
        remote: Some(false),
        auth_option: None,
        username: None,
        password: None,
        pub_key: None,
        private_key: None,
        pass_phrase: None,
        remote_host: None,
        remote_port: None,
        cert_path: None,
        key_path: None,
        use_auth: Some(true),
        manage_users: None,
        account_name: Some("user_name".to_string()),
        account_email: Some("email".to_string()),
        account_password: Some("pass".to_string()),
        account_permission: Some("read".to_string()),
        new_account_name: Some("user_name_new".to_string()),
        jwt_secret: Some("secret".to_string()),
        jwt_duration: Some(10),
        storage: None,
        db_path: None,
    }
}
