use super::utils::{get_file_list, get_folder_list};
use crate::{
    schema::{Context, File, Folder, Message, QueryRoot},
    utils::check_auth_path,
};
use fs_extra::{
    dir::{self},
    move_items,
};
use juniper::FieldResult;
use std::{fs, path::Path};

#[juniper::graphql_object(context = Context)]
impl QueryRoot {
    #[graphql(description = "Returns a list of all files in directory")]
    fn read_file_in_dir(path: String) -> FieldResult<Vec<File>> {
        let path = Path::new(&path);
        check_auth_path(&path)?;
        Ok(get_file_list(&path)?)
    }

    #[graphql(description = "Return the list of all folders in a directory")]
    fn read_dir_in_dir(path: String) -> FieldResult<Vec<Folder>> {
        let path = Path::new(&path);
        check_auth_path(&path)?;
        Ok(get_folder_list(&path)?)
    }

    #[graphql(
        description = "This query can be used to move or rename files. It is also used to rename folders. Do not use to move folders"
    )]
    fn move_or_rename_file(from: String, to: String) -> FieldResult<Message> {
        let from_path = Path::new(&from);
        check_auth_path(&from_path)?;

        let to_path = Path::new(&to);
        check_auth_path(&to_path)?;
        fs::rename(from_path, to_path)?;
        Ok(Message::new(String::from("Action completed successfully")))
    }

    #[graphql(
        description = "This query is specifically for moving a folder or group of folders. It can also be used to move group of files"
    )]
    fn move_folders(from: Vec<String>, to: String) -> FieldResult<Message> {
        let to_path = Path::new(&to);
        check_auth_path(&to_path)?;
        //check if all from destination is permitted diurectory
        for item in from.iter() {
            check_auth_path(&Path::new(&item))?;
        }
        let options = dir::CopyOptions::new();
        move_items(&from, to, &options)?;
        Ok(Message::new(String::from("Action completed successfully")))
    }
}
