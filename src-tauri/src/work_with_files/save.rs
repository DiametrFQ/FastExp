use std::fs::{metadata, read_dir, File, ReadDir};
use std::io::Write;

pub enum DirIs {
    Protect(),
    Open(ReadDir),
}

pub fn find_files_in_dir(target_directory: &str) -> DirIs {
    let Ok(files) = read_dir(target_directory) else {
        return DirIs::Protect();
    };
    DirIs::Open(files)
}

fn find_all_files(target_directory: &str, all_paths: &mut Vec<String>) {
    let DirIs::Open(files) = find_files_in_dir(target_directory) else {
        return;
    };
    // match files {
    //     Protect() -> ,
    //     Open(ReadDir) -> ,
    // }

    for file_entry in files {
        let Ok(entry) = file_entry else { return };
        let file_name = entry.file_name();

        let Some(file_str) = file_name.to_str() else {
            return;
        };
        if file_str.starts_with(".") {
            return;
        }

        let output_line = format!("C:{}*{}\n", target_directory, file_str);
        all_paths.push(output_line);

        let Ok(metadata) = metadata(entry.path()) else {
            return;
        };

        if metadata.is_dir() {
            let new_target_directory = target_directory.to_owned() + file_str + "/";
            find_all_files(&new_target_directory, all_paths);
        }
    }
}

#[tauri::command]
pub fn save_paths_from(directory: &str) {
    let mut all_paths: Vec<String> = vec![];

    find_all_files(&directory, &mut all_paths);

    let save_file = "./files.txt";
    File::create(&save_file)
        .expect("Error with creating file")
        .write(&all_paths.join(" ").as_bytes())
        .expect("Error write file");
}
