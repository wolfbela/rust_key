/*
store all login and password in the app
-   password and login need to be hash
-   name of the section
-   associated link
*/
pub mod login_gestion;
pub mod master_login;

use std::fs::File;
use std::io::prelude::*;

pub fn write_password_into_file(content: &str, path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
