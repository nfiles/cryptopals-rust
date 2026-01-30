use crate::{frequency::Frequency, xor_with_byte};

#[derive(Clone)]
pub struct SingleByteXorCipher {
    pub key: u8,
    pub score: f64,
    pub cleartext: String,
}

pub struct SingleByteXorDecryptor {
    standard_freq: Frequency,
}

impl SingleByteXorDecryptor {
    pub fn with_standard_freq(standard_freq: Frequency) -> SingleByteXorDecryptor {
        SingleByteXorDecryptor { standard_freq }
    }

    pub fn from_corpus(corpus: &str) -> SingleByteXorDecryptor {
        let standard_freq = Frequency::analyze(&corpus);
        SingleByteXorDecryptor { standard_freq }
    }

    pub fn decrypt(self: &Self, encoded: &[u8]) -> Option<SingleByteXorCipher> {
        (0u8..=255u8)
            .filter_map(|key| {
                let xor_bytes: Vec<u8> = xor_with_byte(encoded.iter().cloned(), key).collect();

                // the value must be a valid string
                let cleartext = String::from_utf8(xor_bytes).ok()?;
                let score = self.standard_freq.score_str(&cleartext);

                Some(SingleByteXorCipher {
                    key,
                    score,
                    cleartext,
                })
            })
            .max_by(|cipher1, cipher2| cipher1.score.total_cmp(&cipher2.score))
    }
}
