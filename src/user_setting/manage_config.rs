use std::{io::Error, io::ErrorKind, path::Path};
use toml::{Table, Value};
use toml_edit::{value, Document};

use crate::{auth::bcrypt_util::encrypt_password, cli::Args};
const GRAPH_FS_CONFIG: &str = "./graph_fs.toml";

pub fn add_user(args: &Args) -> Result<(), Error> {
    check_for_config();
    //if no acc name or passqword is provided, terminate program
    if args.account_name.is_none() || args.account_password.is_none() {
        panic!("Please provide a valid acc username and password")
    }
    let acc_user = args.clone().account_name.unwrap();
    let acc_pass = args.clone().account_password.unwrap();

    //parse raw toml config
    let data = read_config_file().unwrap();
    let raw_cfg = &data.parse::<Table>();

    match raw_cfg {
        Ok(config) => {
            //check if username already exist
            if config.get(&acc_user).is_some() {
                log::info!("This username is already being used")
            } else {
                //construct valid toml file
                let mut doc = data.parse::<Document>().expect("invalid doc");
                doc[&acc_user]["name"] = value(&acc_user);
                doc[&acc_user]["password"] = value(encrypt_password(&acc_pass).unwrap());
                if args.account_permission.is_some() {
                    doc[&acc_user]["permission"] = value(args.clone().account_permission.unwrap())
                }
                write_config_file(&doc.to_string()).expect("error writing config");
            }
            Ok(())
        }
        Err(_) => Err(Error::new(ErrorKind::InvalidData, "Invalid config file")),
    }
}

pub fn delete_user(args: &Args) -> Result<(), Error> {
    check_for_config();
    //if no acc name or passqword is provided, terminate program
    if args.account_name.is_none() {
        panic!("Please provide a valid acc username")
    }
    let acc_user = args.clone().account_name.unwrap();

    //parse raw toml config
    let data = read_config_file().unwrap();
    let raw_cfg = &data.parse::<Table>();

    match raw_cfg {
        Ok(config) => {
            //check if username already exist
            if config.get(&acc_user).is_none() {
                log::info!("This username does not exist")
            } else {
                let mut doc = data.parse::<Document>().expect("invalid doc");
                doc.remove(&acc_user);

                write_config_file(&doc.to_string()).expect("error writing config");
            }
            Ok(())
        }
        Err(_) => Err(Error::new(ErrorKind::InvalidData, "Invalid config file")),
    }
}

pub fn edit_user_acc_name(args: &Args) -> Result<(), Error> {
    check_for_config();
    //if no acc name or passqword is provided, terminate program
    if args.account_name.is_none() || args.new_account_name.is_none() {
        panic!("Please provide a valid acc username and password")
    }
    let acc_user = args.clone().account_name.unwrap();
    let new_name = args.clone().new_account_name.unwrap();

    //parse raw toml config
    let data = read_config_file().unwrap();
    let raw_cfg = &data.parse::<Table>();

    match raw_cfg {
        Ok(config) => {
            //check if username already exist
            if config.get(&acc_user).is_none() {
                log::info!("This username does not exist")
            } else {
                //construct valid toml file
                let mut doc = data.parse::<Document>().expect("invalid doc");
                //update the new user profile
                doc[&acc_user]["name"] = value(&args.clone().new_account_name.unwrap());
                //store the profile in a n-> Result<(), Error>  ew variable
                let new_profile = doc.get(&acc_user).unwrap();
                //set the key to the new profile name
                doc[&new_name] = new_profile.clone();
                //delete old user profile
                doc.remove(&acc_user);

                write_config_file(&doc.to_string()).expect("error writing config");
            }
            Ok(())
        }
        Err(_) => Err(Error::new(ErrorKind::InvalidData, "Invalid config file")),
    }
}

pub fn edit_user_acc_password(args: &Args) -> Result<(), Error> {
    check_for_config();
    //if no acc name or passqword is provided, terminate program
    if args.account_name.is_none() || args.account_password.is_none() {
        panic!("Please provide a valid acc username and password")
    }
    let acc_user = args.clone().account_name.unwrap();
    let acc_pass = args.clone().account_password.unwrap();

    //parse raw toml config
    let data = read_config_file().unwrap();
    let raw_cfg = &data.parse::<Table>();

    match raw_cfg {
        Ok(config) => {
            //check if username already exist
            if config.get(&acc_user).is_none() {
                log::info!("This username does not exist")
            } else {
                //construct valid toml file
                let mut doc = data.parse::<Document>().expect("invalid doc");
                doc[&acc_user]["password"] = value(encrypt_password(&acc_pass).unwrap());

                write_config_file(&doc.to_string()).expect("error writing config");
            }
            Ok(())
        }
        Err(_) => Err(Error::new(ErrorKind::InvalidData, "Invalid config file")),
    }
}

pub fn edit_user_acc_priviledge(args: &Args) -> Result<(), Error> {
    check_for_config();
    //if no acc name or passqword is provided, terminate program
    if args.account_name.is_none() || args.account_permission.is_none() {
        panic!("Please provide a valid acc username and password")
    }
    let acc_user = args.clone().account_name.unwrap();
    let acc_permission = args.clone().account_permission.unwrap();

    //parse raw toml config
    let data = read_config_file().unwrap();
    let raw_cfg = &data.parse::<Table>();

    match raw_cfg {
        Ok(config) => {
            //check if username already exist
            if config.get(&acc_user).is_none() {
                log::info!("This username does not exist")
            } else {
                //construct valid toml file
                let mut doc = data.parse::<Document>().expect("invalid doc");
                doc[&acc_user]["permission"] = value(&acc_permission);

                write_config_file(&doc.to_string()).expect("error writing config");
            }
            Ok(())
        }
        Err(_) => Err(Error::new(ErrorKind::InvalidData, "Invalid config file")),
    }
}

pub fn get_user(user: &str) -> Result<Value, Error> {
    //parse raw toml config
    let data = read_config_file().unwrap();
    let raw_cfg = &data.parse::<Table>();
    match raw_cfg {
        Ok(data) => match data.get(user) {
            Some(user) => Ok(user.clone()),
            None => Err(Error::new(ErrorKind::NotFound, "User not found")),
        },
        Err(_) => Err(Error::new(ErrorKind::InvalidData, "Invalid config file")),
    }
}

fn check_for_config() {
    if !Path::new(GRAPH_FS_CONFIG).exists() {
        let toml = "";
        let doc = toml.parse::<Document>().expect("error creating config");
        std::fs::write(GRAPH_FS_CONFIG, doc.to_string().as_bytes())
            .expect("Couldnt write to config file");
    }
}

fn read_config_file() -> std::io::Result<String> {
    let data = std::fs::read_to_string(GRAPH_FS_CONFIG)?;
    Ok(data)
}

fn write_config_file(data: &str) -> std::io::Result<()> {
    Ok(std::fs::write(GRAPH_FS_CONFIG, data.as_bytes())?)
}
