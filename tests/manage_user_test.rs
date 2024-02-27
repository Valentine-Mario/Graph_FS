// use graph_fs::factory::args_factory;
// use graph_fs::user_setting::manage_config::{
//     add_user, delete_user, edit_user_acc_name, edit_user_acc_password, edit_user_acc_priviledge,
//     get_user, GRAPH_FS_CONFIG,
// };
// use graph_fs::db::DBConn;
// use std::path::Path;

// // NOTE: PLEASE ALWAYS RUN TEST ON A SINGLE THREAD WITH THE COMMAND
// // cargo test -- --test-threads=1
// struct CleanUp;

// impl Drop for CleanUp {
//     fn drop(&mut self) {
//         println!("Clean up after running all test");
//         if Path::new(GRAPH_FS_CONFIG).exists() {
//             match std::fs::remove_file(GRAPH_FS_CONFIG) {
//                 Ok(_) => {}
//                 Err(_) => {
//                     println!("error deleting file")
//                 }
//             }
//         }
//     }
// }

// fn add_user_util() -> Result<(), std::io::Error> {
//     let arg = args_factory();
//     add_user(&arg, DBConn::new())
// }

// #[test]
// fn test_add_user() {
//     let add = add_user_util();
//     assert!(add.is_ok());
//     let user = get_user("user_name", DBConn::new());
//     assert!(user.is_ok());
//     let _my_setup = CleanUp;
// }

// #[test]
// fn test_delete_user() {
//     let add = add_user_util();
//     let arg = args_factory();

//     assert!(add.is_ok());
//     let deleted_user = delete_user(&arg.account_name);
//     assert!(deleted_user.is_ok());
//     let user = get_user("user_name");
//     assert!(user.is_err());
//     let _my_setup = CleanUp;
// }

// #[test]
// fn test_edit_user() {
//     let add = add_user_util();
//     let arg = args_factory();

//     assert!(add.is_ok());
//     let edited_user = edit_user_acc_name(&arg);
//     assert!(edited_user.is_ok());
//     let user = get_user("user_name");
//     assert!(user.is_err());
//     let user = get_user("user_name_new");
//     assert!(user.is_ok());
//     let _my_setup = CleanUp;
// }

// #[test]
// fn test_edit_password() {
//     let add = add_user_util();
//     let arg = args_factory();

//     assert!(add.is_ok());
//     let edited_user = edit_user_acc_password(&arg);
//     assert!(edited_user.is_ok());
//     let user = get_user("user_name");
//     assert!(user.is_ok());
//     let _my_setup = CleanUp;
// }

// #[test]
// fn test_edit_priviledge() {
//     let add = add_user_util();
//     let arg = args_factory();

//     assert!(add.is_ok());
//     let edited_user = edit_user_acc_priviledge(&arg);
//     assert!(edited_user.is_ok());
//     let user = get_user("user_name");
//     assert!(user.is_ok());
//     let _my_setup = CleanUp;
// }
