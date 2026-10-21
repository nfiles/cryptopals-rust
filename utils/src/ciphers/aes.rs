use openssl::symm::{Cipher, decrypt};

pub fn aes_128_ecb_decrypt(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_128_ecb();
    decrypt(cipher, key, None, ciphertext).unwrap()
}
