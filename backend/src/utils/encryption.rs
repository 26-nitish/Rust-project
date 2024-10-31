use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::Rng;
use sha2::{Sha256, Digest};
use hex;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

fn derive_key(password: &str) -> [u8; 16] {
    let hash = Sha256::digest(password.as_bytes());
    let mut key = [0u8; 16];
    key.copy_from_slice(&hash[..16]); 
    key
}

pub fn encrypt_message(message: &str, password: &str) -> String {
    let key = derive_key(password);
    let iv: [u8; 16] = rand::thread_rng().gen(); 
    
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).expect("Invalid key/IV length");

    
    let ciphertext = cipher.encrypt_vec(message.as_bytes());
    let mut encrypted_data = iv.to_vec(); 
    encrypted_data.extend(ciphertext); 

    hex::encode(encrypted_data) 
}

pub fn decrypt_message(encrypted_message: &str, password: &str) -> String {
    let key = derive_key(password);
    
  
    let encrypted_data = hex::decode(encrypted_message).expect("Invalid hex string");
    let (iv, ciphertext) = encrypted_data.split_at(16); 
    
    let cipher = Aes128Cbc::new_from_slices(&key, iv).expect("Invalid key/IV length");
    
    
    let decrypted_data = cipher.decrypt_vec(ciphertext).expect("Decryption failed");
    String::from_utf8(decrypted_data).expect("Invalid UTF-8")
}
//   use this function to encrypt and decrypt the message using AES 256 ot sha 256 