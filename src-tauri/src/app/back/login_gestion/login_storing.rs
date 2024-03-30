use serde::{Deserialize, Serialize};
const PATH_OF_LOGINS_FILE: &str = "C:\\Users\\elieu\\AppData\\Local\\rustKey\\logins.json";
pub static mut LOGINS: Vec<Login> = Vec::new();

#[derive(Debug, Deserialize, Serialize)]
pub struct Login {
    pub name: String,
    pub username: String,
    pub password: String,
    pub associated_websites: String, // The websites will be separated by '|' a pipe
}

impl Login {
    fn new(name: &str, username: &str, password: &str, associated_websites: &str) -> Self {
        Self {
            name: name.to_string(),
            password: password.to_string(),
            associated_websites: associated_websites.to_string(),
            username: username.to_string(),
        }
    }
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
This function will take the inputs of the front.
It will create the new login.
It will call `register_new_login` to save the login.
It will add the new login to Logins array.
*/
#[tauri::command(rename_all = "snake_case")]
pub fn create_new_login(name: &str, username: &str, password: &str, associated_websites: &str) {
    println!("----------> create new login");
    unsafe {
        LOGINS.push(Login::new(name, username, password, associated_websites));
        dbg!(&LOGINS);
    }
}

#[tauri::command(renam_all = "snake_case")]
pub fn delete_login(login_id: u16) {}
