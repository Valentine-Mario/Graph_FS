use crate::cli::Args;
use std::io::{Error, ErrorKind};

pub mod manage_config;

pub fn manage_update(args: &Args) -> Result<(), Error> {
    use crate::cli::UserConfig::*;
    match &args.manage_users {
        Some(options) => match options {
            AddUser => manage_config::add_user(args),
            DeleteUser => manage_config::delete_user(args),
            UpdateUserName => manage_config::edit_user_acc_name(args),
            UpdateUserPassword => manage_config::edit_user_acc_password(args),
            UpdateUserPermission => manage_config::edit_user_acc_priviledge(args),
        },
        None => Err(Error::new(
            ErrorKind::InvalidData,
            "No manmagement user option provided",
        )),
    }
}
