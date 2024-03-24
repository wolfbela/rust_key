/*
store all login and password in the app
-   password and login need to be hash
-   name of the section
-   associated link
*/
pub mod file_gestion;
pub mod login_gestion;
pub mod master_login;

use file_gestion::encrypt_content;
use std::fs::File;
use std::io::prelude::*;

const PATH_OF_RUSTKEY_DIRECTORY: &str = "C:\\Users\\elieu\\AppData\\Local\\rustKey";

pub fn write_master_password_into_file(content: &str, path: &str) -> std::io::Result<()> {
    let _ = std::fs::create_dir(PATH_OF_RUSTKEY_DIRECTORY);
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn write_logins_into_file(content: &str, path: &str, key: &[u8]) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(encrypt_content(content, key).as_bytes())?;
    Ok(())
}
