use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::path::Path;

const PATH_OF_LOGINS_FILE: &str = "C:\\Users\\elieu\\AppData\\Local\\rustKey\\logins.json";

#[derive(Debug, Deserialize, Serialize)]
pub struct Login {
    pub name: String,
    pub username: String,
    pub password: String,
    pub associated_websites: String, // The websites will be separated by '|' a pipe
}

impl Default for Login {
    fn default() -> Self {
        Self {
            name: String::from(""),
            username: String::from(""),
            password: String::from(""),
            associated_websites: String::from(""),
        }
    }
}

/*
This function will encrypt the new created login.
After that, the json will be stored in the computer.
*/
fn register_new_login(new_login: Login) {}

/*
This function will take the inputs of the front.
It will create the new login.
It will call `register_new_login` to save the login.
It will add the new login to Logins array.
*/
#[tauri::command(rename_all = "snake_case")]
pub fn create_new_login(name: &str, username: &str, password: &str, associated_websites: &str) {}

/*
This function will load the file where the passwords are located.
It will decypher the file to read out the logins.
It will collect those and put it into the LOGINS variable.
*/
#[tauri::command(rename_all = "snake_case")]
pub fn load_logins() {
    if Path::new(PATH_OF_LOGINS_FILE).exists() {
        let logins_file_content = fs::read_to_string(PATH_OF_LOGINS_FILE).unwrap();
        serde_json::from_str(&logins_file_content).unwrap()
    } else {
        let _ = File::create(PATH_OF_LOGINS_FILE);
        serde_json::from_str("{}").unwrap()
    }
}
