use std::fs;

use super::save::{find_files_in_dir, DirIs};

#[tauri::command]
pub fn get_files_from_dir(target_directory: &str) -> (Vec<String>, Vec<String>) {
    let DirIs::Open(filesRD) = find_files_in_dir(target_directory) else {
        return (
            vec!["This derictory is protected!".to_string()],
            vec!["This derictory is protected!".to_string()],
        );
    };
    let mut files: Vec<String> = vec![];
    let mut dirs: Vec<String> = vec![];

    // let files_name = filesRD.map(|entry| -> String {
    //     let Ok(entry) = entry else {
    //         return "Err".to_string();
    //     };

    //     let file_name = entry.file_name();
    //     let Ok(metadata) = fs::metadata(entry.path()) else {
    //         return "Err".to_string();
    //     };

    //     let Some(file_name) = file_name.to_str() else {
    //         return "NoName?".to_string();
    //     };
    //     if metadata.is_dir() {
    //         return file_name.to_string() + &'d'.to_string();
    //     } else {
    //         return file_name.to_string() + &'f'.to_string();
    //     }
    // });

    // let arr = files_name.filter(|file_name| file_name != "Err").collect();
    for file_rd in filesRD {
        let Ok(entry) = file_rd else {
            break;
        };
        let file_name = entry.file_name();
        let Ok(metadata) = fs::metadata(entry.path()) else {
            break;
        };
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
