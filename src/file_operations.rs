use std::{fs, io};
use std::path::{Path, PathBuf};

use rand::seq::SliceRandom;
use rand::thread_rng;
use thiserror::Error;

pub const BASE_PATH: &str = "./gifs/base.gif";
pub const STARTUP_PATH: &str = "./gifs/startup.gif";
pub const OTHER_PATH: &str = "./gifs/others/";
pub const BLINK_PATH: &str = "./gifs/blinks/";

#[derive(Error, Debug)]
pub enum FileOperationsError {
    #[error("An IO error occurred: {0}")]
    Io(#[from] io::Error),
}


pub fn files_in_directory(dir: &Path) -> Result<Vec<PathBuf>, FileOperationsError> {
    //Modified https://doc.rust-lang.org/std/fs/fn.read_dir.html

    let mut entries = fs::read_dir(dir)?
        .filter_map(|res| {
            if let Ok(entry) = res {
                if entry.metadata().map(|meta| meta.is_file()).unwrap_or(false) {
                    Some(entry.path())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<PathBuf>>();

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::{Path, PathBuf};

    use tempdir::TempDir;

    use super::*;

    #[test]
    fn test_files_in_directory() {
        let temp_dir = TempDir::new("test_files_in_directory").unwrap();
        let dir = temp_dir.path();

        let file_1 = dir.join("file_1.txt");
        let subdir = dir.join("subdir");
        fs::create_dir(subdir.clone()).unwrap();
        let file_2 = subdir.join("file_2.txt");

        fs::File::create(file_1.as_path()).unwrap();
        fs::File::create(file_2.as_path()).unwrap();

        let expected = vec![file_1];
        let result = files_in_directory(dir).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_files_in_nonexistent_directory() {
        let temp_dir = TempDir::new("test_files_in_nonexistent_directory").unwrap();
        let dir = temp_dir.path().join("nonexistent_dir");
        let result = files_in_directory(dir.as_path());
        assert_eq!(result, None);
    }

    #[test]
    fn test_files_in_unreadable_directory() {
        let temp_dir = TempDir::new("test_files_in_unreadable_directory").unwrap();
        let dir = temp_dir.path().join("unreadable_dir");
        fs::create_dir(dir.clone()).unwrap();
        fs::set_permissions(dir.clone(), fs::Permissions::from_mode(0o000)).unwrap();
        let result = files_in_directory(dir.as_path());
        assert_eq!(result, None);
    }
}