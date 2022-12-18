use aes_gcm::{
    aead::{Aead, KeyInit, generic_array::GenericArray},
    Aes256Gcm, Nonce
};
use base64;
use std::str;

pub fn encrypt(msg: &String, key: &String, nonce: &String) -> String {
    let cipher = Aes256Gcm::new(&GenericArray::from_slice(key.as_ref()));
    let nonce = Nonce::from_slice(nonce.as_bytes()); 

    let ciphertext = cipher.encrypt(nonce, msg.as_ref()).unwrap();

    return base64::encode(&ciphertext);
}

pub fn decrypt(msg: &String, key: &String, nonce: &String) -> String {
    let cipher = Aes256Gcm::new(&GenericArray::from_slice(key.as_ref()));
    let nonce = Nonce::from_slice(nonce.as_bytes()); 

    let plaintext = cipher.decrypt(nonce, base64::decode(msg).unwrap().as_slice()).expect("Error on decryption");

    return str::from_utf8(&plaintext).unwrap().to_owned();
}


