use crate::cli::Args;
use std::io::{Error, ErrorKind};

pub mod manage_config;

pub fn manage_update(args: &Args) -> Result<(), Error> {
    use crate::cli::UserConfig::*;
    match &args.manage_users {
        Some(options) => match options {
            AddUser => Ok(()),
            DeleteUser => Ok(()),
            UpdateUserName => Ok(()),
            UpdateUserPassword => Ok(()),
            UpdateUserPermission => Ok(()),
        },
        None => Err(Error::new(
            ErrorKind::InvalidData,
            "No manmagement user option provided",
        )),
    }
}
