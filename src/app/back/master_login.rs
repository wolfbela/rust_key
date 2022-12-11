use ring::pbkdf2;
use std::fs;

/*
This function should verify the master password.
It will apply a PBKDF2 algo on the enter password and will compare it to the stored master password.
*/
#[allow(dead_code)]
pub fn verify_master_password(_password_entered: &String) -> bool {
    true
}
