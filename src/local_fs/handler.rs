use std::{io::Error, path::Path};

use juniper::FieldResult;

use super::utils::{get_file_list, get_folder_list};
use crate::schema::{Context, File, Folder, QueryRoot};

#[juniper::graphql_object(context = Context)]
impl QueryRoot {
    fn read_file_in_dir(path: String) -> FieldResult<Vec<File>> {
        Ok(get_file_list(Path::new(&path))?)
    }

    fn read_dir_in_dir(path: String) -> FieldResult<Vec<Folder>> {
        Ok(get_folder_list(Path::new(&path))?)
    }
}
