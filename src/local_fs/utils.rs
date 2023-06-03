use std::{
    fs,
    io::{Error, ErrorKind},
    path::Path,
};

use crate::schema::{File, Folder};
use fs_extra::dir::{get_dir_content2, DirOptions};

pub fn get_file_list(path: &Path) -> Result<Vec<File>, Error> {
    let file_list = fs::read_dir(path)?
        .into_iter()
        .filter(|r| r.is_ok()) // Get rid of Err variants for Result<DirEntry>
        .map(|r| r.unwrap().path()) // This is safe, since we only have the Ok variants
        .filter(|r| r.is_file()) // Filter out non-files
        .map(|x| {
            let size = get_file_size(&x)?;
            let name = x.to_str().unwrap().to_string();
            let file_type = get_file_type(&name)?;
            let parent_folder = get_parent_folder(&x)?;

            Ok(File::new(name, size, file_type, parent_folder))
        })
        .collect::<Result<Vec<File>, Error>>();
    file_list
}

pub fn get_folder_list(path: &Path) -> Result<Vec<Folder>, Error> {
    let folder_list = fs::read_dir(path)?
        .into_iter()
        .filter(|r| r.is_ok()) // Get rid of Err variants for Result<DirEntry>
        .map(|r| r.unwrap().path()) // This is safe, since we only have the Ok variants
        .filter(|r| r.is_dir()) // Filter out non-files
        .map(|x| {
            let name = x.to_str().unwrap().to_string();
            let content_length = get_dir_content_length(&x)?;
            let parent_folder = get_parent_folder(&x)?;

            Ok(Folder::new(name, content_length, parent_folder))
        })
        .collect::<Result<Vec<Folder>, Error>>();

    folder_list
}

fn get_file_size(path: &Path) -> Result<f64, Error> {
    Ok((path.metadata()?.len()) as f64)
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
            return Ok((dir_len + file_len - 1) as i32);
        }
        Err(_) => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "error getting folder content",
            ))
        }
    }
}
