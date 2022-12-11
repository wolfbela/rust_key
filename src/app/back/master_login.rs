use ring::{digest, pbkdf2};
use std::{fs, num::NonZeroU32};

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA512;
const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;

enum Error {
    WrongUserOrPassword,
}

struct MasterPassword {
    hashed_password: String,
    salt: [u8; 16],
}

/*
This function will pars the file where the hash of the master password is store.
It will fille the MasterPassword struct with the content of it.
*/
fn file_to_master_password(path_of_file: String) -> MasterPassword {
    let file_content = fs::read_to_string(path_of_file).unwrap();
    let parsed_file = json::parse(&file_content.as_str());

    todo!()
}

/*
This function should verify the master password.
It will apply a PBKDF2 algo on the enter password and will compare it to the stored master password.
*/
#[allow(dead_code)]
pub fn verify_master_password(password_entered: &String) -> bool {
    let reference_password: MasterPassword = file_to_master_password(String::from(""));
    let NB_ITERATION: NonZeroU32 = NonZeroU32::new(1024).unwrap();

    /*
    This function  will verify the password entered.
    -   if the result is an Error, it means that we entered the wrong password.
    */
    pbkdf2::verify(
        PBKDF2_ALG,
        NB_ITERATION,
        &reference_password.salt,
        password_entered.as_bytes(),
        reference_password.hashed_password.as_bytes(),
    )
    .map_err(|_| Error::WrongUserOrPassword)
    .is_ok()
}
