use std::{fs, io};
use std::path::{Path, PathBuf};

use log::error;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// The path to the base animation
pub const BASE_PATH: &str = "./gifs/base.gif";

/// The path to the startup animation
pub const STARTUP_PATH: &str = "./gifs/startup.gif";

/// The path to the directory with all the animations
pub const OTHER_PATH: &str = "./gifs/others/";

/// The path to the directory, with all the blink animations
pub const BLINK_PATH: &str = "./gifs/blinks/";

#[derive(Error, Debug)]
pub enum FileOperationsError {
    #[error("An IO error occurred: {0}")]
    Io(#[from] io::Error),
}

#[derive(Serialize, Deserialize, Debug)]
/// Structure the JSON playlist files get parsed to
pub struct Playlist {
    ///Indicates that a JSON file contains a playlist
    data_type: String,

    ///Paths as strings to the animation that should get played in given order
    pub entries: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
/// Structure JSON palettes get parsed to
pub struct Palette {
    ///Indicates that a JSON file contains a palette
    data_type: String,

    ///Entries are hex color strings
    pub colors: Vec<String>,
}

/// Read the given palette at the given path into a `Palette`
/// # Input
/// * A `PathBuf` to the path to the palette file
/// # Output
/// A `Palette` containing a vector of colors as strings
pub fn read_palette(path: &PathBuf) -> Result<Palette, serde_json::error::Error> {
    match fs::read_to_string(path) {
        Ok(data) => {
            let palette: Palette = serde_json::from_str(&data)?;
            Ok(palette)
        }
        Err(e) => {
            let message = format!("Can't read palette: {}", e.to_string());
            error!("{}", message);
            panic!("{}", message);
            //todo: Error::new(serde_json::error::Category::Data, "Can't read palette") //use default palette then
        }
    }
}

/// Read the given playlist at the given path into a Playlist
/// # Input
/// * A `PathBuf` to the path to the playlist file
/// # Output
/// A `Playlist` containing a vector of paths to animations
pub fn read_playlist(path: &PathBuf) -> Result<Playlist, serde_json::error::Error> {
    match fs::read_to_string(path) {
        Ok(data) => {
            let playlist: Playlist = serde_json::from_str(&data)?;
            Ok(playlist)
        }
        Err(e) => {
            let message = format!("Can't read playlist: {}", e.to_string());
            error!("{}", message);
            panic!("{}", message);
            //Error::new(serde_json::error::Category::Data, "Can't read playlist")
        }
    }
}

/// List all files in a given directory
///
/// # Input
/// A `Path` to the directory
///
/// # Output
/// A `Result<Vec<PathBuf>, FileOperationsError>, where
/// * `Vec<PathBuf>` is the list of all files within the directory
/// * `FileOperationsError` is thrown, when the directory cannot be read
///
/// # Credits
/// This is a modified version of one of the [std::fs::read_dir Examples](https://doc.rust-lang.org/std/fs/fn.read_dir.html#examples)
pub fn files_in_directory(dir: &Path) -> Result<Vec<PathBuf>, FileOperationsError> {
    let entries = fs::read_dir(dir)?
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
    fn test_read_palette_ok() {
        let read_palette = read_palette(&PathBuf::from("test_palette.json")).unwrap();
        assert_eq!(read_palette.data_type, "palette");
        assert_eq!(read_palette.colors, vec!["FF0000", "00FF00", "0000FF"]);
    }

    #[test]
    fn test_read_playlist_ok() {
        let read_playlist = read_playlist(&PathBuf::from("test_playlist.json")).unwrap();
        assert_eq!(read_playlist.data_type, "playlist");
        assert_eq!(read_playlist.entries, vec![
            "./gifs/others/coin eyes.gif",
            "./gifs/others/colorful.gif",
            "./gifs/others/loading.gif",
            "./gifs/others/portal_eyes.gif",
        ]);
    }
}