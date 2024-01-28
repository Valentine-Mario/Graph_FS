use super::utils::{get_file_list, get_folder_list};
use crate::{
    fs_module::graphql_write_access,
    schema::{Context, File, Folder, Message},
    utils::check_auth_path,
};
use base64::{engine::general_purpose, Engine as _};
use fs_extra::{
    copy_items,
    dir::{self},
    move_items,
};
use juniper::FieldResult;
use std::{
    fs,
    fs::File as RFile,
    io::{Seek, SeekFrom, Write},
    path::Path,
};
pub struct LocalFsQuery;

#[juniper::graphql_object(context = Context)]
impl LocalFsQuery {
    #[graphql(description = "Returns a list of all files in directory")]
    fn read_file_in_dir(path: String) -> FieldResult<Vec<File>> {
        let path = Path::new(&path);
        check_auth_path(path)?;
        Ok(get_file_list(path)?)
    }

    #[graphql(description = "Return the list of all folders in a directory")]
    fn read_dir_in_dir(path: String) -> FieldResult<Vec<Folder>> {
        let path = Path::new(&path);
        check_auth_path(path)?;
        Ok(get_folder_list(path)?)
    }
}

pub struct LocalFsMutation;

#[juniper::graphql_object(context = Context)]
impl LocalFsMutation {
    #[graphql(description = "This mutation can be used to rename files or folders")]
    fn rename_item(context: &Context, from: String, to: String) -> FieldResult<Message> {
        if !graphql_write_access(context) {
            return Ok(Message::new(String::from(
                "Unauthorized to perform write operation",
            )));
        }
        let from_path = Path::new(&from);
        check_auth_path(from_path)?;

        let to_path = Path::new(&to);
        check_auth_path(to_path)?;
        fs::rename(from_path, to_path)?;
        Ok(Message::new(String::from("Item renamed successfully")))
    }

    #[graphql(description = "This mutation is used for moving file(s) or folder(s)")]
    fn move_item(context: &Context, from: Vec<String>, to: String) -> FieldResult<Message> {
        if !graphql_write_access(context) {
            return Ok(Message::new(String::from(
                "Unauthorized to perform write operation",
            )));
        }
        let to_path = Path::new(&to);
        check_auth_path(to_path)?;
        // Check if all from destination is permitted diurectory
        for item in from.iter() {
            check_auth_path(Path::new(&item))?;
        }
        let options = dir::CopyOptions::new();
        move_items(&from, to, &options)?;
        Ok(Message::new(String::from("Item moved successfully")))
    }

    #[graphql(description = "This mutation is to copy an item or group of items")]
    fn copy_item(context: &Context, from: Vec<String>, to: String) -> FieldResult<Message> {
        if !graphql_write_access(context) {
            return Ok(Message::new(String::from(
                "Unauthorized to perform write operation",
            )));
        }
        let to_path = Path::new(&to);
        check_auth_path(to_path)?;
        // Check if all from destination is permitted diurectory
        for item in from.iter() {
            check_auth_path(Path::new(&item))?;
        }
        let options = dir::CopyOptions::new();
        copy_items(&from, to, &options)?;
        Ok(Message::new(String::from("Items copied successfully")))
    }

    #[graphql(description = "Delete directory")]
    fn delete_dir(context: &Context, path: String) -> FieldResult<Message> {
        if !graphql_write_access(context) {
            return Ok(Message::new(String::from(
                "Unauthorized to perform write operation",
            )));
        }
        let path = Path::new(&path);
        check_auth_path(path)?;
        fs::remove_dir_all(path)?;
        Ok(Message::new(String::from("Directory deleted successfully")))
    }

    #[graphql(description = "Delete file")]
    fn delete_file(context: &Context, path: String) -> FieldResult<Message> {
        if !graphql_write_access(context) {
            return Ok(Message::new(String::from(
                "Unauthorized to perform write operation",
            )));
        }
        let path = Path::new(&path);
        check_auth_path(path)?;
        fs::remove_file(path)?;
        Ok(Message::new(String::from("File deleted successfully")))
    }

    #[graphql(description = "Create directory")]
    fn create_dir(context: &Context, path: String) -> FieldResult<Message> {
        if !graphql_write_access(context) {
            return Ok(Message::new(String::from(
                "Unauthorized to perform write operation",
            )));
        }
        let path = Path::new(&path);
        check_auth_path(path)?;
        fs::create_dir_all(path)?;
        Ok(Message::new(String::from("Directory created successfully")))
    }

    #[graphql(description = "Create file")]
    fn create_file(context: &Context, path: String) -> FieldResult<Message> {
        if !graphql_write_access(context) {
            return Ok(Message::new(String::from(
                "Unauthorized to perform write operation",
            )));
        }
        let path = Path::new(&path);
        check_auth_path(path)?;
        RFile::create(path)?;
        Ok(Message::new(String::from("File created successfully")))
    }

    #[graphql(
        description = "Update a file content at a seek position. For large file, use the upload endpoint. Payload should be in base64 encoding"
    )]
    fn update_file(
        context: &Context,
        path: String,
        seek: i32,
        payload: String,
    ) -> FieldResult<Message> {
        if !graphql_write_access(context) {
            return Ok(Message::new(String::from(
                "Unauthorized to perform write operation",
            )));
        }
        let path = Path::new(&path);
        check_auth_path(path)?;
        let bytes = general_purpose::STANDARD.decode(payload)?;
        let mut file = fs::OpenOptions::new()
            .read(true)
            .append(true)
            .create(false)
            .open(path)?;
        file.seek(SeekFrom::Start(seek as u64))?;

        file.write_all(&bytes)?;
        Ok(Message::new(String::from("File updated successfully")))
    }
}
