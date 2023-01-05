use std::fs::File;
use std::io::prelude::*;

use ring::aead::{Nonce, NonceSequence, NONCE_LEN};

const PATH_OF_NONCES_FILE: &str = "C:\\Users\\elieu\\AppData\\Local\\rustKey-nonces.json";

pub struct MyNonceSequence {
    pub nonces: Vec<Nonce>,
}

/*
A nonce sequence is a sequence of unique numbers wich are made to identifies blocs in the CHACHA20.
Here is the implementation of the structure of the Nonce Sequence
*/
impl NonceSequence for MyNonceSequence {
    fn advance(&mut self) -> Result<Nonce, ring::error::Unspecified> {
        match self.nonces.pop() {
            Some(nonce) => Ok(nonce),
            None => Err(ring::error::Unspecified),
        }
    }
}

impl MyNonceSequence {
    /*
    We need to reload the nonces when the application is reloading to access the stored logins again.
    */
    pub fn save_nonce_sequence(nonces: &Vec<Nonce>) -> std::io::Result<()> {
        /*
        First of all, we need to change the Vec<Nonce> to a vector of slices of u8.
        We can't serialize Vec<node> so we need to change it to a serializable type.
        */
        let vec_nonces_values: Vec<&[u8; NONCE_LEN]> =
            nonces.into_iter().map(|x| x.as_ref()).collect();

        /*
        Now come the serialization of the Vec into a string.
        */
        let content_nonces = serde_json::to_string(&vec_nonces_values).unwrap();
        let mut file = File::create(PATH_OF_NONCES_FILE)?;
        file.write_all(content_nonces.as_bytes())?;
        Ok(())
    }
}
