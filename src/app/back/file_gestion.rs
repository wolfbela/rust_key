use ring::aead::*;

struct NonceSequenceTest {
    nonces: Vec<u8>,
}

impl NonceSequence for NonceSequenceTest {
    fn advance(&mut self) -> Result<Nonce, ring::error::Unspecified> {
        todo!()
    }
}

/*
in CHACHA20 the text is separate in blocks of 64 bytes.
each of those blocks is made of the plain text and a tag which ensure that the bloc has no issues of modification or alteration
    this tag is générated by the POLY1305 function.
the goal of the algorithme is to take the whole block, multiplies is by the 'encryption_key'.
Also, each block is associated with a counter wich ensure an encryption wich is unique.
*/
#[allow(dead_code)]
pub fn encrypt_content(content: &str, key: &[u8], nonces: Nonce) -> String {
    /*
    to start, will create the encryption_key from the general key gave by the hashing of the master password
    */
    let _encryption_key = SealingKey::new(
        UnboundKey::new(&CHACHA20_POLY1305, &key).unwrap(),
        NonceSequenceTest { nonces: vec![] },
    );
    let mut vec_content = content.as_bytes().to_vec();

    /*
    Here, we create the necessary space for the concatenation of the tag with the block
    */
    for _ in 0..CHACHA20_POLY1305.tag_len() {
        vec_content.push(0);
    }

    todo!()
}

#[allow(dead_code)]
pub fn decrypt_content(_content: &str) -> String {
    todo!()
}
