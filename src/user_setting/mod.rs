use crate::{cli::Args, db};
use std::io::{Error, ErrorKind};

pub mod manage_config;

pub async fn manage_update(args: &Args) -> Result<(), Error> {
    use crate::cli::UserConfig::*;
    let pool = db::DBConn::new().await.unwrap();
    match &args.manage_users {
        Some(options) => match options {
            AddUser => manage_config::add_user(args.clone(), &pool).await,
            DeleteUser => manage_config::delete_user(&args.account_email, &pool).await,
            UpdateUserName => manage_config::edit_user_acc_name(args.clone(), &pool).await,
            UpdateUserPassword => manage_config::edit_user_acc_password(args.clone(), &pool).await,
            UpdateUserPermission => {
                manage_config::edit_user_acc_priviledge(args.clone(), &pool).await
            }
        },
        None => Err(Error::new(
            ErrorKind::InvalidData,
            "No manmagement user option provided",
        )),
    }
}
