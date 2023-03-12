use crate::app::back::write_master_password_into_file;
use rand::{rngs::StdRng, Rng, SeedableRng};
use ring::{digest, pbkdf2};
use serde::{Deserialize, Serialize};
use std::{fs, num::NonZeroU32, str};

static ALGORITHME: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA512;
const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
const PATH_OF_MASTER_FILE: &str = "C:\\Users\\elieu\\AppData\\Local\\rustKey.json";

enum Error {
    WrongUserOrPassword,
}

#[derive(Debug, Deserialize, Serialize)]
struct MasterPassword {
    hashed_password: Vec<u8>,
    salt: [u8; 32],
}

/*
This function will serialize the MasterPassword struct into a json.
After that, the json will be store in the computer.
*/
#[allow(dead_code)]
pub fn register_master_password(new_master_password: &str) {
    let mut struct_master_password = MasterPassword {
        hashed_password: Vec::with_capacity(CREDENTIAL_LEN),
        salt: [0; 32],
    };
    let nb_iteration: NonZeroU32 = NonZeroU32::new(1024).unwrap();

    /*
    Creation of the salt.
    The goal of the salt is to counter the rainbow table with a random string associate with the master password.
    To do so, it's generate a new random generator with StdRng::from_entropy().
    After that, it use the rng to fill the salt variable.
    */
    let mut rng = StdRng::from_entropy();
    rng.fill(&mut struct_master_password.salt);

    /*
    Creation of the hashed of the password password.
    Use of a pbkdf2 function with the salt that we created before and the master password that we entered.
    */
    let mut hashed_password_tmp = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        ALGORITHME,
        nb_iteration,
        &struct_master_password.salt,
        &new_master_password.as_bytes(),
        &mut hashed_password_tmp,
    );

    struct_master_password.hashed_password = hashed_password_tmp.to_vec();

    /*
    Serialization of the struct master_password and write the structure to a file.
    In case of error during the writing, panic with a clear message.
    */
    let serialized_struct = serde_json::to_string(&struct_master_password).unwrap();
    match write_master_password_into_file(serialized_struct.as_str(), PATH_OF_MASTER_FILE) {
        Ok(_) => return,
        Err(_) => panic!("Could not write the data to a file !!"),
    }
}

/*
This function will parse the file where the hash of the master password is store.
It will fille the MasterPassword struct with the content of it.
*/
fn file_to_master_password(path_of_file: &str) -> MasterPassword {
    let file_content = fs::read_to_string(path_of_file).unwrap();
    serde_json::from_str(&file_content).unwrap()
}

/*
This function should verify the master password.
It will apply a PBKDF2 algo on the enter password and will compare it to the stored master password.
*/
#[allow(dead_code)]
pub fn verify_master_password(password_entered: &str) -> bool {
    let reference_password: MasterPassword = file_to_master_password(PATH_OF_MASTER_FILE);
    let nb_iteration: NonZeroU32 = NonZeroU32::new(1024).unwrap();

    /*
    This function  will verify the password entered.
    -   if the result is an Error, it means that we entered the wrong password.
    */
    pbkdf2::verify(
        ALGORITHME,
        nb_iteration,
        &reference_password.salt,
        password_entered.as_bytes(),
        &reference_password.hashed_password,
    )
    .map_err(|_| Error::WrongUserOrPassword)
    .is_ok()
}

/*
Creation of the sealing passwords key.

The key is the correcte pasword with a 500 times hashing with SHA256.
The goal of this is the automaticly generate the sealing passwords key only when the correct password is entered.

With that, it won't be possible for a malicious user to still the key and decrypt the logins store in the app.
*/
#[allow(dead_code)]
fn sealing_password_logins(password_entered: &str) -> [u8; 32] {
    let reference_password: MasterPassword = file_to_master_password(PATH_OF_MASTER_FILE);
    let mut hashed_key = [0u8; ring::digest::SHA256_OUTPUT_LEN];
    let nb_iteration: NonZeroU32 = NonZeroU32::new(500).unwrap();

    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        nb_iteration,
        &reference_password.salt,
        &password_entered.as_bytes(),
        &mut hashed_key,
    );

    hashed_key
}
