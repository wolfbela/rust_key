/*
store all login and password in the app
-   password and login need to be hash
-   name of the section
-   associated link
*/
pub mod encryption_gestion;
pub mod login_gestion;
pub mod master_login;

use encryption_gestion::encrypt_content;
use login_gestion::login_storing::LOGINS;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;

use self::encryption_gestion::decrypt_content;

const PATH_OF_RUSTKEY_DIRECTORY: &str = "C:\\Users\\elieu\\AppData\\Local\\rustKey";
const PATH_OF_LOGINS_FILE: &str = "C:\\Users\\elieu\\AppData\\Local\\rustKey\\logins.json";

pub fn write_master_password_into_file(content: &str, path: &str) -> std::io::Result<()> {
    match std::fs::create_dir(PATH_OF_RUSTKEY_DIRECTORY) {
        Ok(_) => {
            println!("Rust Key directory created");
        }
        Err(_) => {}
    }

    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn logins_json_generator() -> String {
    unsafe { serde_json::to_string(&LOGINS).unwrap() }
}

/*
This function will take the LOGINS variable and serialize its content.
It will serialize to write it in the logins file.
*/
#[tauri::command(rename_all = "snake_case")]
pub fn write_logins_into_file(key: &[u8]) {
    println!("----------> Write login into file");
    let mut file = File::create(PATH_OF_LOGINS_FILE).unwrap();

    /*
    Serialize the logins to json to write it in the file.
    Encrypt it with the encrypt_content function.
    */
    let content = &logins_json_generator();
    let content_post_encyption = &encrypt_content(&content, key);

    file.write_all(encrypt_content(&content.as_str(), key).as_bytes())
        .unwrap();
}

fn load_in_logins_var(decrypted_content: &String) {
    unsafe {
        LOGINS = serde_json::from_str(decrypted_content).unwrap();
    }
}

/*
This function will load the file where the passwords are located.
It will decypher the file to read out the logins.
It will collect those and put it into the LOGINS variable.
*/
#[tauri::command(rename_all = "snake_case")]
pub fn load_logins(master_key: &str) -> String {
    println!("--------------> load logins");
    if Path::new(PATH_OF_LOGINS_FILE).exists() {
        let logins_file_content = fs::read_to_string(PATH_OF_LOGINS_FILE).unwrap();

        match logins_file_content.len() != 0 {
            true => {
                let decrypted_content = decrypt_content(master_key.as_bytes());
                dbg!(&decrypted_content);
                load_in_logins_var(&decrypted_content);
                decrypted_content
            }
            false => "{}".to_string(),
        }
    } else {
        let _ = File::create(PATH_OF_LOGINS_FILE);
        "{}".to_string()
    }
}
