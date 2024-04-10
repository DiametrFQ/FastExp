use std::fs;
use std::path::Path;

#[tauri::command]
fn find_files(directory: &str) {
    
    let Ok(files) = fs::read_dir(target_directory) else {
        todo!()
    };

    for file_entry in files {
        let Ok(entry) = file_entry {
            todo!()
        };

        let file_name = entry.file_name();

        let Some(file_str) = file_name.to_str() else {
            todo!()
        };

        // if file_str.starts_with(".") {
        //     todo!()
        // }

        let file_path = entry.path();
        let file_path_str = file_path.to_str().unwrap_or("");
        let output_line = format!("C:{}*{}\n", target_directory, file_str);

        if let Err(err) = fs::write("./files.txt", output_line) {
            eprintln!("Error writing to file: {}", err);
        } else {
            println!("The file has been saved!");
        }

        println!("{}", file_str);

        let Ok(metadata) = fs::metadata(file_path) else {
            todo!()
        };

        if metadata.is_dir() {
            let finded_dir = target_directory.to_owned() + file_str + "/";
            find_files(&finded_dir);
        }
    }
}

#[tauri::command]
fn find_in_dir(target_directory: &str) {
    let mut vec = Vec::new();

    if !Path::new("./files.txt").exists() {
        if let Err(err) = fs::write("./files.txt", "") {
            eprintln!("Error creating file: {}", err);
            return Err(vec);
        }
    }

    let Ok(files) = fs::read_dir(target_directory);

    files
}
