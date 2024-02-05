use std::fs;

#[tauri::command]
pub fn find_files(directory: &str) -> Vec<String> {
    let mut all_paths: Vec<String> = vec![];
    find(directory, &mut all_paths);

    all_paths
}

fn find(target_directory: &str, all_paths: &mut Vec<String>) {
    if fs::read_dir(target_directory).is_err() {
        return;
    };

    let files = fs::read_dir(target_directory).unwrap();

    for file_entry in files {
        let Ok(entry) = file_entry else { return };
        let file_name = entry.file_name();

        let Some(file_str) = file_name.to_str() else {
            return;
        };
        // if file_str.starts_with(".") {
        //     return;
        // }
        // if(file_str.){

        // }

        let output_line = format!("C:{}*{}\n", target_directory, file_str);
        all_paths.push(output_line);

        let Ok(metadata) = fs::metadata(entry.path()) else {
            return;
        };

        if metadata.is_dir() {
            let new_target_directory = target_directory.to_owned() + file_str + "/";
            find(&new_target_directory, all_paths);
        }
    }
}
