use openssl::{base64, symm};
use std::str;

pub fn encrypt(msg: &String, key: &String, nonce: &String) -> String {
    let cipher = symm::Cipher::aes_256_cbc();
    let iv = nonce.as_bytes(); 

    let ciphertext = symm::encrypt(cipher, key.as_bytes(), Some(iv), msg.as_bytes()).unwrap();

    return base64::encode_block(&ciphertext);
}

pub fn decrypt(msg: &String, key: &String, nonce: &String) -> String {
    let cipher = symm::Cipher::aes_256_cbc();
    let iv = nonce.as_bytes(); 
    let decoded_b64 = base64::decode_block(msg).unwrap();

    let plaintext = symm::decrypt(cipher, key.as_bytes(), Some(iv), &decoded_b64).expect("Error on decryption");

    return str::from_utf8(&plaintext).unwrap().to_owned();
}
