use log::info;
use std::path::PathBuf;
use std::{env, fs};

pub fn create_temp_directory(dir_name: &str) -> std::io::Result<PathBuf> {
    let temp_dir_path = env::current_dir()?.join(dir_name);
    fs::create_dir(&temp_dir_path)?;

    Ok(temp_dir_path)
}

pub fn rm_file(file_path: &PathBuf, rm_dir: bool) -> std::io::Result<()> {
    if file_path.exists() {
        info!("Removing test workflow file: {}", file_path.display());
        fs::remove_file(&file_path).expect("Failed to remove test workflow file");
    }
    if rm_dir {
        if let Some(parent) = file_path.parent() {
            if parent.exists() {
                info!("Removing parent directory: {}", parent.display());
                fs::remove_dir_all(parent)?;
            }
        }
    }
    Ok(())
}
