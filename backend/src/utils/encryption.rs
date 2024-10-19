use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::Rng;
use hex;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

pub fn encrypt_message(message: &str, password: &str) -> String {

    // TODO: Encrypt the message
    hex::encode("".as_bytes())
}

pub fn decrypt_message(encrypted_message: &str, password: &str) -> String {
   
    // TODO: Decrypt the message
    "".to_string()
}