use core::f64;

use crate::ciphers::frequency::{CharSet, FrequencyMap};
use crate::xor_with_byte;

pub struct SingleByteXorCipher {
    pub key: u8,
    pub cleartext: String,
}

pub struct SingleByteXorDecryptor {
    charset: CharSet,
    standard_freq: FrequencyMap,
}

impl SingleByteXorDecryptor {
    pub fn from_corpus(charset: CharSet, corpus: &str) -> SingleByteXorDecryptor {
        let standard_freq = charset.build_freq_from_corpus(&corpus);
        SingleByteXorDecryptor {
            charset,
            standard_freq,
        }
    }

    pub fn decrypt(self: &Self, encoded: &[u8]) -> Option<SingleByteXorCipher> {
        let mut best_sln: Option<SingleByteXorCipher> = None;
        let mut best_score = f64::MAX;

        for key in 0u8..=255u8 {
            let xor_bytes: Vec<u8> = xor_with_byte(encoded.iter().copied(), key).collect();

            // the resulting value must be a valid string
            let xor_string = match String::from_utf8(xor_bytes) {
                Ok(result) => result,
                Err(_) => continue,
            };

            let score = self.charset.compare_freq(
                &self.standard_freq,
                &self.charset.build_freq_from_corpus(&xor_string),
            );
            if score < best_score {
                best_sln = Some(SingleByteXorCipher {
                    key,
                    cleartext: xor_string,
                });
                best_score = score;
            }
        }

        best_sln
    }
}
