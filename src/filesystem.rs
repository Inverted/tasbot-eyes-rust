use std::fs;

fn files_in_directory(dir: &str) -> Vec<String> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir).expect("failed to read directory") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                files.push(entry.file_name().into_string().unwrap());
            }
        }
    }
    files
}