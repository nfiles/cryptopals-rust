use std::iter;

use openssl::symm::{Cipher, decrypt};

pub struct Pkcs7Padding {
    blocksize: usize,
}

impl Pkcs7Padding {
    pub fn create(blocksize: usize) -> Result<Pkcs7Padding, String> {
        if blocksize > 255 {
            Result::Err("cannot pad to size higher than 255".into())
        } else {
            Result::Ok(Pkcs7Padding { blocksize })
        }
    }

    pub fn pad(&self, input: &[u8]) -> Vec<u8> {
        let padding_size = self.blocksize - (input.len() % self.blocksize);
        let padding_byte = u8::try_from(padding_size).expect("invalid padding byte");
        let padding = iter::repeat(padding_byte).take(padding_size);
        input.iter().cloned().chain(padding).collect()
    }

    pub fn unpad<'a>(&self, input: &'a [u8]) -> Result<&'a [u8], String> {
        let padding_size: usize = match input.last() {
            Some(size) => (*size).into(),
            // return an empty slice, but index it from the input so that it has the expected lifetime
            None => return Result::Ok(&input[0..0]),
        };

        if padding_size > input.len() {
            return Result::Err(format!(
                "invalid padding: expected padding length({}) is longer than input ({})",
                padding_size,
                input.len()
            ));
        }

        let pad_length = input
            .into_iter()
            .rev()
            .take_while(|&&x| usize::from(x) == padding_size)
            .count();

        if pad_length != padding_size {
            return Result::Err(format!(
                "invalid padding: padding byte ({}) does not equal padding length ({})",
                padding_size, pad_length
            ));
        }

        Result::Ok(&input[..(input.len() - pad_length)])
    }
}

pub fn aes_128_ecb_decrypt(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_128_ecb();
    decrypt(cipher, key, None, ciphertext).unwrap()
}
