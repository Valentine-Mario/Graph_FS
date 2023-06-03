use std::{io::Error, io::ErrorKind, path::Path};

pub fn check_auth_path(child: &Path) -> Result<bool, Error> {
    let args = crate::cli::Args::new();
    let parent = Path::new(&args.authorized_path);
    if child.starts_with(parent) {
        return Ok(true);
    } else {
        return Err(Error::new(
            ErrorKind::PermissionDenied,
            "unauthorized to access directory",
        ));
    }
}
