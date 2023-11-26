use std::path::Path;

use juniper::FieldResult;

use crate::{
    fs_module::utils::{get_remote_file_list, get_remote_folder_list},
    schema::{Context, File, Folder, Message},
    utils::check_auth_path,
};
pub struct RemoteFsQuery;

#[juniper::graphql_object(context = Context)]
impl RemoteFsQuery {
    #[graphql(description = "Returns a list of all files in directory")]
    fn read_file_in_dir(context: &Context, path: String) -> FieldResult<Vec<File>> {
        let path = Path::new(&path);
        check_auth_path(&path)?;
        let sftp = context.sess.as_ref().unwrap().sftp()?;
        Ok(get_remote_file_list(&path, sftp)?)
    }

    #[graphql(description = "Returns a list of all directories in directory")]
    fn read_dir_in_dir(context: &Context, path: String) -> FieldResult<Vec<Folder>> {
        let path = Path::new(&path);
        check_auth_path(&path)?;
        let sftp = context.sess.as_ref().unwrap().sftp()?;
        Ok(get_remote_folder_list(&path, &sftp)?)
    }
}

pub struct RemoteFsMutation;

#[juniper::graphql_object(context = Context)]
impl RemoteFsMutation {
    #[graphql(description = "Create file")]
    fn create_file(context: &Context, path: String) -> FieldResult<Message> {
        let path = Path::new(&path);
        check_auth_path(&path)?;
        let sftp = context.sess.as_ref().unwrap().sftp()?;
        sftp.create(path)?;
        let return_msg = format!("{} created successfully", path.to_str().unwrap());
        Ok(Message::new(String::from(return_msg)))
    }

    #[graphql(
        description = "Create directory. Set mode optionally, would default to allow user read and write without sudo"
    )]
    fn create_dir(context: &Context, path: String, mode: Option<i32>) -> FieldResult<Message> {
        let path = Path::new(&path);
        check_auth_path(&path)?;
        let sftp = context.sess.as_ref().unwrap().sftp()?;
        // Use 1000 as mode if none provided
        sftp.mkdir(path, mode.unwrap_or(1000))?;
        let return_msg = format!("{} created successfully", path.to_str().unwrap());
        Ok(Message::new(String::from(return_msg)))
    }

    #[graphql(description = "Delete a file")]
    fn delete_file(context: &Context, path: String) -> FieldResult<Message> {
        let path = Path::new(&path);
        check_auth_path(&path)?;
        let sftp = context.sess.as_ref().unwrap().sftp()?;
        sftp.unlink(path)?;
        let return_msg = format!("{} deleted successfully", path.to_str().unwrap());
        Ok(Message::new(String::from(return_msg)))
    }

    #[graphql(description = "Delete a folder")]
    fn delete_dir(context: &Context, path: String) -> FieldResult<Message> {
        let path = Path::new(&path);
        check_auth_path(&path)?;
        let sftp = context.sess.as_ref().unwrap().sftp()?;
        sftp.rmdir(path)?;
        let return_msg = format!("{} deleted successfully", path.to_str().unwrap());
        Ok(Message::new(String::from(return_msg)))
    }

    #[graphql(description = "Rename a file or folder, also used to move item")]
    fn rename_item(context: &Context, from: String, to: String) -> FieldResult<Message> {
        let from_path = Path::new(&from);
        check_auth_path(&from_path)?;

        let to_path = Path::new(&to);
        check_auth_path(&to_path)?;
        let sftp = context.sess.as_ref().unwrap().sftp()?;
        sftp.rename(from_path, to_path, None)?;
        Ok(Message::new(String::from("Operation successful")))
    }
}
