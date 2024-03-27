use crate::schema::MySshFileType;
use resolve_path::PathResolveExt;
use ssh2::FileType;
use std::{fmt, io::Error, io::ErrorKind, path::Path};

pub fn check_auth_path(child: &Path) -> Result<bool, Error> {
    let args = crate::cli::Args::new();
    let path = &args
        .authorized_path
        .expect("please specify authorized path");
    let parent = path.resolve();
    if child.starts_with(parent) {
        Ok(true)
    } else {
        Err(Error::new(
            ErrorKind::PermissionDenied,
            "unauthorized to access directory",
        ))
    }
}

// Map SSH filetype to my filetype
pub fn map_enum(input: FileType) -> MySshFileType {
    match input {
        FileType::BlockDevice => MySshFileType::BlockDevice,
        FileType::CharDevice => MySshFileType::CharDevice,
        FileType::Directory => MySshFileType::Directory,
        FileType::NamedPipe => MySshFileType::NamedPipe,
        FileType::RegularFile => MySshFileType::RegularFile,
        FileType::Socket => MySshFileType::Socket,
        FileType::Symlink => MySshFileType::Symlink,
        FileType::Other(_) => MySshFileType::Other,
    }
}

impl fmt::Display for MySshFileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MySshFileType::BlockDevice => write!(f, "Block Device"),
            MySshFileType::CharDevice => write!(f, "Char Device"),
            MySshFileType::Directory => write!(f, "Directory"),
            MySshFileType::NamedPipe => write!(f, "Named Pipe"),
            MySshFileType::Other => write!(f, "Other"),
            MySshFileType::RegularFile => write!(f, "Regular File"),
            MySshFileType::Symlink => write!(f, "Symlink"),
            MySshFileType::Socket => write!(f, "Socket"),
        }
    }
}
