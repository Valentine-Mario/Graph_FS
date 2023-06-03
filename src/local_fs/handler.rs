use super::utils::{get_file_list, get_folder_list};
use crate::{
    schema::{Context, File, Folder, QueryRoot},
    utils::check_auth_path,
};
use juniper::FieldResult;
use std::path::Path;

#[juniper::graphql_object(context = Context)]
impl QueryRoot {
    fn read_file_in_dir(path: String) -> FieldResult<Vec<File>> {
        let path = Path::new(&path);
        check_auth_path(&path)?;
        Ok(get_file_list(&path)?)
    }

    fn read_dir_in_dir(path: String) -> FieldResult<Vec<Folder>> {
        let path = Path::new(&path);
        check_auth_path(&path)?;
        Ok(get_folder_list(&path)?)
    }
}
