use core::f64;

use crate::ciphers::frequency::Frequency;
use crate::xor_with_byte;

pub struct SingleByteXorCipher {
    pub key: u8,
    pub score: f64,
    pub cleartext: String,
}

pub struct SingleByteXorDecryptor {
    standard_freq: Frequency,
}

impl SingleByteXorDecryptor {
    pub fn from_corpus(corpus: &str) -> SingleByteXorDecryptor {
        let standard_freq = Frequency::from_corpus(&corpus);
        SingleByteXorDecryptor { standard_freq }
    }

    pub fn decrypt(self: &Self, encoded: &[u8]) -> Option<SingleByteXorCipher> {
        (0u8..=255u8)
            .filter_map(|key| {
                let xor_bytes: Vec<u8> = xor_with_byte(encoded.iter().copied(), key).collect();

                // the value must be a valid string
                let cleartext = String::from_utf8(xor_bytes).ok()?;

                let score = self
                    .standard_freq
                    .compare_with(&Frequency::from_corpus(&cleartext));

                Some(SingleByteXorCipher {
                    key,
                    score,
                    cleartext,
                })
            })
            .min_by(|cipher1, cipher2| f64::total_cmp(&cipher1.score, &cipher2.score))
    }
}
