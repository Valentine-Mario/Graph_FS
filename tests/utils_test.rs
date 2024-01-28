use graph_fs::fs_module::utils::*;

// NOTE: PLEASE ALWAYS RUN TEST ON A SINGLE THREAD WITH THE COMMAND
// cargo test -- --test-threads=1


#[test]
fn test_get_file_list() {
   let list= get_file_list(
    std::env::current_dir()
                .unwrap()
                .as_path()
   );
   assert_eq!(list.is_ok(), true);
   assert_eq!(list.unwrap().len()>1, true)
}


#[test]
fn test_get_folder_list(){
    let list= get_folder_list(
        std::env::current_dir()
                    .unwrap()
                    .as_path()
       );
       assert_eq!(list.is_ok(), true);
       assert_eq!(list.unwrap().len()>1, true)
}