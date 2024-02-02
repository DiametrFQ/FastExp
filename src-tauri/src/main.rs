// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fastexp::work_with_files::{get_files_name, read, save};

use std::env;
// use std::Result::Err;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            save::save_paths_from,
            read::text_file,
            get_files_name::get_files_from_dir,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
