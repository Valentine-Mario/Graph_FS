use graph_fs::factory::args_factory;
use graph_fs::user_setting::manage_config::{
    add_user, delete_user, edit_user_acc_name, edit_user_acc_password, edit_user_acc_priviledge,
    get_user, GRAPH_FS_CONFIG,
};
use std::path::Path;

// NOTE: PLEASE ALWAYS RUN TEST ON A SINGLE THREAD WITH THE COMMAND
// cargo test -- --test-threads=1
struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        println!("Clean up after running all test");
        if Path::new(GRAPH_FS_CONFIG).exists() {
            match std::fs::remove_file(GRAPH_FS_CONFIG) {
                Ok(_) => {}
                Err(_) => {
                    println!("error deleting file")
                }
            }
        }
    }
}

fn add_user_util() -> Result<(), std::io::Error> {
    let arg = args_factory();
    add_user(&arg)
}

#[test]
fn test_add_user() {
    let add = add_user_util();
    assert_eq!(add.is_ok(), true);
    let user = get_user("user_name");
    assert_eq!(user.is_ok(), true);
    let _my_setup = CleanUp;
}

#[test]
fn test_delete_user() {
    let add = add_user_util();
    let arg = args_factory();

    assert_eq!(add.is_ok(), true);
    let deleted_user = delete_user(&arg.account_name);
    assert_eq!(deleted_user.is_ok(), true);
    let user = get_user("user_name");
    assert_eq!(user.is_err(), true);
    let _my_setup = CleanUp;
}

#[test]
fn test_edit_user() {
    let add = add_user_util();
    let arg = args_factory();

    assert_eq!(add.is_ok(), true);
    let edited_user = edit_user_acc_name(&arg);
    assert_eq!(edited_user.is_ok(), true);
    let user = get_user("user_name");
    assert_eq!(user.is_err(), true);
    let user = get_user("user_name_new");
    assert_eq!(user.is_ok(), true);
    let _my_setup = CleanUp;
}

#[test]
fn test_edit_password() {
    let add = add_user_util();
    let arg = args_factory();

    assert_eq!(add.is_ok(), true);
    let edited_user = edit_user_acc_password(&arg);
    assert_eq!(edited_user.is_ok(), true);
    let user = get_user("user_name");
    assert_eq!(user.is_ok(), true);
    let _my_setup = CleanUp;
}

#[test]
fn test_edit_priviledge() {
    let add = add_user_util();
    let arg = args_factory();

    assert_eq!(add.is_ok(), true);
    let edited_user = edit_user_acc_priviledge(&arg);
    assert_eq!(edited_user.is_ok(), true);
    let user = get_user("user_name");
    assert_eq!(user.is_ok(), true);
    let _my_setup = CleanUp;
}
