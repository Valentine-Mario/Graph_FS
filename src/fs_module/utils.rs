use std::{
    fs,
    io::{Error, ErrorKind},
    path::Path,
    time::SystemTime,
};

use crate::{
    schema::{File, Folder},
    utils::map_enum,
};
use fs_extra::dir::{get_dir_content2, DirOptions};
use std::net::TcpStream;

use ssh2::{Session, Sftp};

use crate::cli::Args;

// Lib for local FS

pub fn get_file_list(path: &Path) -> Result<Vec<File>, Error> {
    
    fs::read_dir(path)?
        .filter(|r| r.is_ok()) // Get rid of Err variants for Result<DirEntry>
        .map(|r| r.unwrap().path()) // This is safe, since we only have the Ok variants
        .filter(|r| r.is_file()) // Filter out non-files
        .map(|x| {
            let size = get_file_size(&x)?;
            let name = x.to_str().unwrap().to_string();
            let file_type = get_file_type(&name)?;
            let parent_folder = get_parent_folder(&x)?;
            let last_modified = last_modified(&x)?;

            Ok(File::new(
                name,
                size,
                file_type,
                parent_folder,
                last_modified,
            ))
        })
        .collect::<Result<Vec<File>, Error>>()
}

pub fn get_folder_list(path: &Path) -> Result<Vec<Folder>, Error> {
    

    fs::read_dir(path)?
        .filter(|r| r.is_ok()) // Get rid of Err variants for Result<DirEntry>
        .map(|r| r.unwrap().path()) // This is safe, since we only have the Ok variants
        .filter(|r| r.is_dir()) // Filter out non-files
        .map(|x| {
            let name = x.to_str().unwrap().to_string();
            let content_length = get_dir_content_length(&x)?;
            let parent_folder = get_parent_folder(&x)?;

            Ok(Folder::new(name, content_length, parent_folder))
        })
        .collect::<Result<Vec<Folder>, Error>>()
}

fn get_file_size(path: &Path) -> Result<f64, Error> {
    Ok((path.metadata()?.len()) as f64)
}

fn last_modified(path: &Path) -> Result<f64, Error> {
    let meta = path.metadata()?;
    if let Ok(time) = meta.modified() {
        let time = time.duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let subsec_millis = time.as_secs() as f64;
        Ok(subsec_millis)
    } else {
        Ok(0 as f64)
    }
}

fn get_file_type(name: &str) -> Result<String, Error> {
    let format = file_format::FileFormat::from_file(name)?.name().to_string();
    Ok(format)
}

fn get_parent_folder(path: &Path) -> Result<String, Error> {
    match path.parent() {
        Some(parent) => Ok(parent.to_str().unwrap().to_string()),
        None => Ok(Path::new("").to_str().unwrap().to_string()),
    }
}

fn get_dir_content_length(path: &Path) -> Result<i32, Error> {
    let mut options = DirOptions::new();
    options.depth = 1;
    match get_dir_content2(path, &options) {
        Ok(directories) => {
            let dir_len = directories.directories.len();
            let file_len = directories.files.len();
            Ok((dir_len + file_len - 1) as i32)
        }
        Err(_) => {
            Err(Error::new(
                ErrorKind::InvalidInput,
                "Error getting folder content",
            ))
        }
    }
}

// Lib for remote FS

// SSH connection
pub fn connection(args: &Args, mut sess: Session) -> Result<Session, std::io::Error> {
    let args = args.clone();
    let url_host = format!(
        "{}:{}",
        args.remote_host.unwrap(),
        args.remote_port.unwrap()
    );
    let tcp = TcpStream::connect(url_host)?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    match args.auth_option {
        Some(auth_option) => {
            match auth_option {
                crate::cli::AuthOption::Agent => {
                    sess.userauth_agent(&args.username.unwrap()).unwrap();
                }
                crate::cli::AuthOption::Password => {
                    sess.userauth_password(&args.username.unwrap(), &args.password.unwrap())
                        .unwrap();
                }
                crate::cli::AuthOption::PubkeyFile => {
                    sess.userauth_pubkey_file(
                        &args.username.unwrap(),
                        Some(Path::new(&args.pub_key.unwrap())),
                        Path::new(&args.private_key.unwrap()),
                        Some(&args.pass_phrase.unwrap()),
                    )
                    .unwrap();
                }
            }
            Ok(sess)
        }
        None => Err(Error::new(
            ErrorKind::InvalidData,
            "No auth option provided",
        )),
    }
}

pub fn get_remote_file_list(path: &Path, sftp: Sftp) -> Result<Vec<File>, Error> {
    

    sftp
        .readdir(path)?
        .into_iter()
        .filter(|r| r.1.is_file())
        .map(|x| {
            let name = x.0.to_str().unwrap().to_string();
            let size = x.1.size.unwrap_or(0) as f64;
            let file_type = map_enum(x.1.file_type()).to_string();
            let parent_folder = get_parent_folder(&x.0)?;
            let last_modified = x.1.mtime.unwrap_or(0) as f64;

            Ok(File::new(
                name,
                size,
                file_type,
                parent_folder,
                last_modified,
            ))
        })
        .collect::<Result<Vec<File>, Error>>()
}

pub fn get_remote_folder_list(path: &Path, sftp: &Sftp) -> Result<Vec<Folder>, Error> {
    

    sftp
        .readdir(path)?
        .into_iter()
        .filter(|r| r.1.is_dir())
        .map(|x| {
            let name = x.0.to_str().unwrap().to_string();
            let content_length = get_remote_dir_content_length(&x.0, sftp)? as i32;
            let parent_folder = get_parent_folder(&x.0)?;

            Ok(Folder::new(name, content_length, parent_folder))
        })
        .collect::<Result<Vec<Folder>, Error>>()
}

fn get_remote_dir_content_length(path: &Path, sftp: &Sftp) -> Result<usize, Error> {
    Ok(sftp.readdir(path)?.len())
}
