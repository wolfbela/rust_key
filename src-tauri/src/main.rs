#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod app;

use app::back::login_gestion::login_storing::{create_new_login, delete_login};
use app::back::master_login::{is_file_here, register_master_password, verify_master_password};
use app::back::{load_logins, write_logins_into_file};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            register_master_password,
            create_new_login,
            delete_login,
            verify_master_password,
            write_logins_into_file,
            is_file_here,
            load_logins,
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
