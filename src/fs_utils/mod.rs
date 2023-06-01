use std::path::{Path, PathBuf};

pub fn get_dir() -> Result<PathBuf, std::io::Error> {
    let env_path = std::env::var("MOUNT");
    match env_path {
        Ok(env_path) => {
            println!("env {}", env_path);
            return Ok(Path::new(&env_path).to_path_buf());
        }
        Err(_) => {
            let current_path = std::env::current_dir().expect("Error getting current directory");
            return Ok(current_path);
        }
    }
}
