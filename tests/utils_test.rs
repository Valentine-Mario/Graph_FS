use graph_fs::auth::{bcrypt_util::*, jwt::*};
use graph_fs::factory::args_factory;
use graph_fs::fs_module::utils::{get_file_list, get_folder_list};

// NOTE: PLEASE ALWAYS RUN TEST ON A SINGLE THREAD WITH THE COMMAND
// cargo test -- --test-threads=1

#[test]
fn test_get_file_list() {
    let list = get_file_list(std::env::current_dir().unwrap().as_path());
    assert!(list.is_ok());
    assert!(!list.unwrap().is_empty())
}

#[test]
fn test_get_folder_list() {
    let list = get_folder_list(std::env::current_dir().unwrap().as_path());
    assert!(list.is_ok());
    assert!(!list.unwrap().is_empty())
}

#[test]
fn test_bcrypt_password() {
    let password = encrypt_password("password");
    assert!(password.is_ok());
    let cmp = compare_password(&password.unwrap(), "password");
    assert!(cmp.is_ok());
    assert!(cmp.unwrap())
}

#[test]
fn test_jwt_create() {
    let arg = args_factory();
    let token = create_token("user_id", 1, arg.clone().jwt_secret);
    assert!(token.is_ok());
    let validate = validate_token(&token.unwrap(), arg.jwt_secret);
    assert!(validate.is_ok())
}

#[test]
fn test_jwt_decode() {
    let arg = args_factory();
    let token = create_token("user_id", 1, arg.clone().jwt_secret);
    assert!(token.is_ok());
    let decoded = decode_token(&token.unwrap(), arg.jwt_secret);
    assert!(decoded.is_ok());
    assert_eq!(decoded.unwrap(), "user_id")
}
