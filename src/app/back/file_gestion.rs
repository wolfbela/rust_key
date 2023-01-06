pub mod nonce_sequence_gestion;

use std::fs;

use rand::{rngs::StdRng, Rng, SeedableRng};
use ring::aead::*;

use crate::app::front::PATH_OF_LOGINS_FILE;
use nonce_sequence_gestion::{load_nonces, MyNonceSequence};

/*
Each block has to be of 64 bits. each block need a nonce to have unique encryption. A nonce (Number once = number used one time).
Here, the goal is to generate a random nonce for each block and put all of those into res.
*/
fn create_nonce_sequence(plane_text: &[u8]) -> Vec<Nonce> {
    let nb_of_blocks = plane_text.len() / 64 + 1;

    let mut res: Vec<Nonce> = Vec::new();

    /*
    The, tmp_nonce_arr is an temporary variable used to create one nonce.
    */
    let mut tmp_nonce_arr: [u8; 12] = [0x0; 12];
    for _ in 0..nb_of_blocks {
        /*
        We fill the temporary variable with random number before puting it in the nonce vector.
        */
        let mut rng = StdRng::from_entropy();
        rng.fill(&mut tmp_nonce_arr);
        res.push(Nonce::assume_unique_for_key(tmp_nonce_arr));
    }

    res
}

/*
in CHACHA20 the text is separate in blocks of 64 bytes.
each of those blocks is made of the plain text and a tag which ensure that the bloc has no issues of modification or alteration
    this tag is générated by the POLY1305 function.
the goal of the algorithme is to take the whole block, multiplies is by the 'encryption_key'.
Also, each block is associated with a counter wich ensure an encryption wich is unique.
*/
pub fn encrypt_content(content: &str, key: &[u8]) -> String {
    /*
    Instantiation of nonce sequence, it is necessary to get the encryption key
    */
    let nonce_sequence = MyNonceSequence {
        nonces: create_nonce_sequence(content.as_bytes()),
    };

    /*
    Save the nonces to decrypt the logins when the app start again.
    */
    match MyNonceSequence::save_nonce_sequence(&nonce_sequence.nonces) {
        Ok(_) => dbg!("Nonces Well Saved"),
        Err(_) => dbg!("Could not save Nonces"),
    };

    /*
    To start, will create the encryption_key from the general key gave by the hashing of the master password
    */
    let mut encryption_key = SealingKey::new(
        UnboundKey::new(&CHACHA20_POLY1305, &key).unwrap(),
        nonce_sequence,
    );

    /*
    Here, we create the necessary space for the concatenation of the tag with the block
    */
    let mut vec_content = content.as_bytes().to_vec();
    for _ in 0..CHACHA20_POLY1305.tag_len() {
        vec_content.push(0);
    }

    let additional_data: [u8; 0] = [];

    /*
    Encryption of the file
    The in_out variable has the file contente in clear.
    The funtion will encrypte the file content and put it into the vec_content var again.
    */
    match SealingKey::seal_in_place_append_tag(
        &mut encryption_key,
        Aad::from(additional_data),
        &mut vec_content,
    ) {
        Ok(_) => dbg!("Success encryption !"),
        Err(_) => panic!("Error encryption T-T"),
    };

    serde_json::to_string(&vec_content).unwrap()
}

/*
When openning the application, that will load the logins after the authentication.
To do that, it will reverse the CHACHA20 encryption by using the same key and the same nonces.
*/
pub fn decrypt_content(key: &[u8]) -> String {
    /*
    Creation of the nonces sequences using the nonces of the last session.
    To get those, we're using the file where we stored it.
    */
    let nonce_sequence = MyNonceSequence {
        nonces: load_nonces(),
    };

    /*
    Creation of the decryption key using the hash of the master password (after login)
    */
    let mut decryption_key = OpeningKey::new(
        UnboundKey::new(&CHACHA20_POLY1305, &key).unwrap(),
        nonce_sequence,
    );

    let additional_data: [u8; 0] = [];

    let file_content = fs::read_to_string(PATH_OF_LOGINS_FILE).unwrap();
    /*
    Change the file content value into the array of encrypted valued in content var.
    */
    let mut content: Vec<u8> = serde_json::from_str(&file_content).unwrap();

    /*
    Decryption of the file
    The in_out variable has the file content in encrypted.
    The funtion will decrypte the content and put it into the content var again.
    */
    match decryption_key.open_in_place(Aad::from(&additional_data), &mut content) {
        Ok(_) => dbg!("Decryption succeeded"),
        Err(_) => dbg!("Decryption failed"),
    };

    /*
    Strip of the content to stop it after the first '\0'.
    Some value which are not a string could be still stored after the value of the string.
    */
    let stipped_content = content
        .iter()
        .take_while(|&&x| x != 0)
        .map(|&x| x)
        .collect::<Vec<u8>>();

    /*
    Change of the Vec<u8> into a string.
    */
    String::from_utf8(stipped_content).unwrap()
}
