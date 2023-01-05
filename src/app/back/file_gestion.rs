pub mod nonce_sequence_gestion;

use rand::{rngs::StdRng, Rng, SeedableRng};
use ring::aead::*;

use nonce_sequence_gestion::MyNonceSequence;

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

#[allow(dead_code)]
pub fn decrypt_content(_content: &str) -> String {
    todo!()
}
