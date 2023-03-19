#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod app;

use app::back::master_login::{is_file_here, register_master_password, verify_master_password};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            register_master_password,
            verify_master_password,
            is_file_here,
            greet
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    dbg!(name);
    format!("Hello, {}!", name)
}
