use std::fs;
use std::path::{Path, PathBuf};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn files_in_directory(dir: &Path) -> Option<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir).ok()? {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                files.push(path);
            }
        }
    }

    Some(files)
}

//todo: could this be turned into a iterator extension --> makes not much sense, lol
pub fn shuffle<T>(mut vec: Vec<T>) -> Vec<T>{
    vec.shuffle(&mut thread_rng());
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::{Path, PathBuf};
    use tempdir::TempDir;

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