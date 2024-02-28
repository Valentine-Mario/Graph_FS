use graph_fs::db::DBConn;
use graph_fs::factory::args_factory;
use graph_fs::user_setting::manage_config::{
    add_user, delete_user, edit_user_acc_name, edit_user_acc_password, edit_user_acc_priviledge,
    get_user,
};
use std::path::Path;

// // NOTE: PLEASE ALWAYS RUN TEST ON A SINGLE THREAD WITH THE COMMAND
// // cargo test -- --test-threads=1
struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        println!("Clean up after running all test");
        if Path::new("./graph_fs.db").exists() {
            match std::fs::remove_file("./graph_fs.db") {
                Ok(_) => {}
                Err(_) => {
                    println!("error deleting file")
                }
            }
        }
    }
}

async fn add_user_util() -> Result<(), std::io::Error> {
    let arg = args_factory();
    add_user(arg, &DBConn::new().await.unwrap()).await.unwrap();
    Ok(())
}

#[actix_rt::test]
async fn test_add_user() {
    let add = add_user_util().await;
    assert!(add.is_ok());
    let user = get_user("email", &DBConn::new().await.unwrap()).await;
    assert!(user.is_ok());
    let _my_setup = CleanUp;
}

#[actix_rt::test]
async fn test_delete_user() {
    let add = add_user_util().await;
    let arg = args_factory();

    assert!(add.is_ok());
    let dbase = &DBConn::new().await.unwrap();
    let deleted_user = delete_user(&arg.account_email, &dbase).await;
    assert!(deleted_user.is_ok());
    let user = get_user("email", &dbase).await;
    assert!(user.is_ok());
    assert!(user.unwrap().len() == 0);
    let _my_setup = CleanUp;
}

#[actix_rt::test]
async fn test_edit_user() {
    let add = add_user_util().await;
    let arg = args_factory();

    assert!(add.is_ok());
    let dbase = &DBConn::new().await.unwrap();

    let edited_user = edit_user_acc_name(arg, &dbase).await;
    assert!(edited_user.is_ok());
    let user = get_user("email", &dbase).await;
    assert!(user.is_ok());
    assert!(user.as_ref().unwrap().len() > 0);
    assert!(user.unwrap()[0].name == "user_name_new");
    let _my_setup = CleanUp;
}

#[actix_rt::test]
async fn test_edit_password() {
    let add = add_user_util().await;
    let arg = args_factory();

    assert!(add.is_ok());
    let dbase = &DBConn::new().await.unwrap();
    let edited_user = edit_user_acc_password(arg, &dbase).await;
    assert!(edited_user.is_ok());
    let user = get_user("user_name", &dbase).await;
    assert!(user.is_ok());
    let _my_setup = CleanUp;
}

#[actix_rt::test]
async fn test_edit_priviledge() {
    let add = add_user_util().await;
    let arg = args_factory();

    assert!(add.is_ok());
    let dbase = &DBConn::new().await.unwrap();

    let edited_user = edit_user_acc_priviledge(arg, &dbase).await;
    assert!(edited_user.is_ok());
    let user = get_user("user_name", &dbase).await;
    assert!(user.is_ok());
    let _my_setup = CleanUp;
}
