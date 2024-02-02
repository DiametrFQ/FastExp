use super::save::{find_files_in_dir, DirIs};

#[tauri::command]
pub fn get_files_from_dir(target_directory: &str) -> Vec<String> {
    let DirIs::Open(files) = find_files_in_dir(target_directory) else {
        return vec!["This derictory is protected!".to_string()];
    };
    let files_name = files.map(|entry| -> String {
        let Ok(entry) = entry else {
            return "Err".to_string();
        };
        let file_name = entry.file_name();
        let Some(file_name) = file_name.to_str() else {
            return "NoName?".to_string();
        };

        file_name.to_string()
    });

    files_name.filter(|file_name| file_name != "Err").collect()
}
