use std::{io::Error, io::ErrorKind};

use crate::{auth::bcrypt_util::encrypt_password, cli::Args, db::DBConn, schema::User};

pub async fn add_user(args: Args, conn: &DBConn) -> Result<(), Error> {
    //if no acc name or passqword is provided, terminate program
    if args.account_name.is_none()
        || args.account_password.is_none()
        || args.account_email.is_none()
    {
        panic!("Please provide a valid acc username, email and password")
    }
    let acc_user = args.account_name.unwrap();
    let acc_pass = args.account_password.unwrap();
    let acc_email = args.account_email.unwrap();
    let permission = args.account_permission;
    let hashed = encrypt_password(&acc_pass).expect("error hashing password");

    match conn
        .create_user(&acc_user, &acc_email, &hashed, permission)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            let error = format!("{:?}", err);
            Err(Error::new(ErrorKind::InvalidData, error))
        }
    }
}

pub async fn delete_user(email: &Option<String>, conn: &DBConn) -> Result<(), Error> {
    if email.is_none() {
        panic!("Please provide a valid acc username")
    }

    match conn.delete_user(&email.clone().unwrap()).await {
        Ok(_) => Ok(()),
        Err(err) => {
            let error = format!("{:?}", err);
            Err(Error::new(ErrorKind::Other, error))
        }
    }
}

pub async fn edit_user_acc_name(args: Args, conn: &DBConn) -> Result<(), Error> {
    if args.account_email.is_none() || args.new_account_name.is_none() {
        panic!("Please provide a valid acc username and password")
    }
    let acc_email = args.account_email.unwrap();
    let new_name = args.new_account_name.unwrap();

    match conn.update_user_name(&new_name, &acc_email).await {
        Ok(_) => Ok(()),
        Err(err) => {
            let error = format!("{:?}", err);
            Err(Error::new(ErrorKind::InvalidData, error))
        }
    }
}

pub async fn edit_user_acc_password(args: Args, conn: &DBConn) -> Result<(), Error> {
    if args.account_name.is_none() || args.account_password.is_none() {
        panic!("Please provide a valid acc username and password")
    }
    let acc_email = args.account_email.unwrap();
    let acc_pass = args.account_password.unwrap();
    let hash = encrypt_password(&acc_pass).expect("error hashing password");

    match conn.update_user_password(&hash, &acc_email).await {
        Ok(_) => Ok(()),
        Err(err) => {
            let error = format!("{:?}", err);
            Err(Error::new(ErrorKind::InvalidData, error))
        }
    }
}

pub async fn edit_user_acc_priviledge(args: Args, conn: &DBConn) -> Result<(), Error> {
    if args.account_email.is_none() || args.account_permission.is_none() {
        panic!("Please provide a valid acc email and password")
    }
    let acc_email = args.account_email.unwrap();
    let acc_permission = args.account_permission.unwrap();

    match conn
        .update_user_permission(&acc_permission, &acc_email)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            let error = format!("{:?}", err);
            Err(Error::new(ErrorKind::InvalidData, error))
        }
    }
}

pub async fn get_user(email: &str, conn: &DBConn) -> Result<Vec<User>, Error> {
    match conn.get_user_by_email(email).await {
        Ok(user) => Ok(user),
        Err(err) => {
            let error = format!("{:?}", err);
            Err(Error::new(ErrorKind::InvalidData, error))
        }
    }
}
