use std::fs;

use super::save::{find_files_in_dir, DirIs};

#[tauri::command]
pub fn get_files_from_dir(target_directory: &str) -> (Vec<String>, Vec<String>) {
    let DirIs::Open(filesrd) = find_files_in_dir(target_directory) else {
        return (
            vec!["This derictory is protected!".to_string()],
            vec!["This derictory is protected!".to_string()],
        );
    };
    let mut files: Vec<String> = vec![];
    let mut dirs: Vec<String> = vec![];

    for file_rd in filesrd {
        let Ok(entry) = file_rd else {
            break;
        };
        let Ok(metadata) = fs::metadata(entry.path()) else {
            break;
        };
        let file_name = entry.file_name();
        let Some(file_name) = file_name.to_str() else {
            break;
        };
        if metadata.is_dir() {
            dirs.push(file_name.to_string())
        } else {
            files.push(file_name.to_string())
        }
    }
    return (dirs, files);
}
