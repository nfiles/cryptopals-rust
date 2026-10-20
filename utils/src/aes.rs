use openssl::symm::{Cipher, decrypt};

pub fn aes_128_ecb_decrypt(ciphertext: &[u8], key: &[u8]) -> String {
    let cipher = Cipher::aes_128_ecb();
    String::from_utf8(decrypt(cipher, key, None, ciphertext).unwrap()).unwrap()
}
