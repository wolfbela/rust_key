use rand::{rngs::StdRng, Rng, SeedableRng};
use ring::{digest, pbkdf2};
use serde::{Deserialize, Serialize};
use std::{fs, num::NonZeroU32};

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA512;
const _CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;

enum Error {
    WrongUserOrPassword,
}

#[derive(Deserialize, Serialize)]
struct MasterPassword {
    hashed_password: String,
    salt: vec![u8; 32],
}

/*
This function will serialize the MasterPassword struct into a json.
After that, the json will be store in the computer.
*/
#[allow(dead_code)]
pub fn register_master_password(new_master_password: &String) {
    let mut struct_master_password = MasterPassword {
        hashed_password: String::from(""),
        salt: [0; 32],
    };

    /*
    Creation of the salt.
    The goal of the salt is to counter the rainbow table with a random string associate with the master password.
    To do so, it's generate a new random generator with StdRng::from_entropy().
    After that, it use the rng to fill the salt variable.
    */
    let mut rng = StdRng::from_entropy();
    rng.fill(&mut struct_master_password.salt);

    /*
    This part of programme will concatenaite the master password with a salt.
    */
    let mut unhashe_string = new_master_password.clone();
    unhashe_string.push_str(
        String::from_utf8(struct_master_password.salt.to_vec())
            .unwrap()
            .as_str(),
    );

    let serialized_struct = serde_json::to_string(&struct_master_password).unwrap();
}

/*
This function will pars the file where the hash of the master password is store.
It will fille the MasterPassword struct with the content of it.
*/
fn file_to_master_password(path_of_file: String) -> MasterPassword {
    let file_content = fs::read_to_string(path_of_file).unwrap();
    serde_json::from_str(&file_content).unwrap()
}

/*
This function should verify the master password.
It will apply a PBKDF2 algo on the enter password and will compare it to the stored master password.
*/
#[allow(dead_code)]
pub fn verify_master_password(password_entered: &String) -> bool {
    let reference_password: MasterPassword = file_to_master_password(String::from(""));
    let nb_iteration: NonZeroU32 = NonZeroU32::new(1024).unwrap();

    /*
    This function  will verify the password entered.
    -   if the result is an Error, it means that we entered the wrong password.
    */
    pbkdf2::verify(
        PBKDF2_ALG,
        nb_iteration,
        &reference_password.salt,
        password_entered.as_bytes(),
        reference_password.hashed_password.as_bytes(),
    )
    .map_err(|_| Error::WrongUserOrPassword)
    .is_ok()
}
