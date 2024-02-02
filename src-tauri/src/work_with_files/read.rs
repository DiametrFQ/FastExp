use std::fs::File;
use std::io::Read;
// use download;

use super::save;

#[tauri::command]
pub fn text_file(path: &str) -> Vec<Vec<String>> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            save::save_paths_from("/");
            File::open(path).expect("Can't open created file. Cringe")
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Read error");

    parse_file(&contents)
}

fn parse_file(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|line| {
            line.split('*')
                .map(|s| s.to_string()) // Convert &str to String
                .collect::<Vec<String>>()
        })
        .collect()
}
