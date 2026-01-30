use std::iter::repeat_with;

use crate::{
    analysis::get_hamming_distance, ciphers::single_byte_xor::SingleByteXorDecryptor,
    frequency::Frequency, xor_with_key,
};

pub fn get_probable_key_sizes(encrypted: &[u8]) -> Vec<usize> {
    // to get a better average, we want to examine a consistent multiple of the key length
    let max_key_length = 40;
    let key_length_multiplier = (encrypted.len() / max_key_length).min(3);

    let mut possible_key_sizes: Vec<_> = (2..=max_key_length)
        // only consider keys that are shorter than the ciphertext
        .filter(|&length| length < encrypted.len())
        .map(|length| {
            let chunk_length = length * key_length_multiplier;
            let chunk1 = get_chunk(encrypted, chunk_length, 0);
            let chunk2 = get_chunk(encrypted, chunk_length, 1);
            let distance = get_hamming_distance(&chunk1, &chunk2);
            let normalized = distance as f64 / chunk_length as f64;

            (normalized, length)
        })
        .collect();

    possible_key_sizes.sort_by(|(left, _), (right, _)| left.total_cmp(&right));

    let num_possible_key_sizes = (possible_key_sizes.len() / 3).max(40);
    possible_key_sizes
        .iter()
        .take(num_possible_key_sizes)
        .map(|(_, keysize)| keysize)
        .cloned()
        .collect()
}

pub fn get_vertical_slices(source: &[u8], size: usize) -> Vec<Vec<u8>> {
    (0..size)
        .map(|col| {
            let mut row = 0;
            repeat_with(|| {
                let idx = size * row + col;
                row += 1;
                idx
            })
            .take_while(|&idx| idx < source.len())
            .map(|idx| source[idx])
            .collect()
        })
        .collect()
}

fn get_chunk<'a>(encrypted: &'a [u8], size: usize, index: usize) -> &'a [u8] {
    let start = size * index;
    let end = size * (index + 1);

    if start >= encrypted.len() {
        // kinda hacky way to make a slice with the expected lifetime
        &encrypted[0..0]
    } else if end >= encrypted.len() {
        &encrypted[start..encrypted.len()]
    } else {
        &encrypted[start..end]
    }
}

#[derive(Clone)]
pub struct RepeatingKeyXorCipher {
    pub key: Vec<u8>,
    pub score: f64,
}

pub struct RepeatingKeyXorDecryptor {
    standard_freq: Frequency,
    single_byte_xor_decryptor: SingleByteXorDecryptor,
}

impl RepeatingKeyXorDecryptor {
    pub fn from_corpus(corpus: &str) -> RepeatingKeyXorDecryptor {
        let standard_freq = Frequency::analyze(&corpus);
        let single_byte_xor_decryptor =
            SingleByteXorDecryptor::with_standard_freq(standard_freq.clone());

        RepeatingKeyXorDecryptor {
            standard_freq,
            single_byte_xor_decryptor,
        }
    }

    pub fn decrypt(self: &Self, encrypted: &[u8]) -> Option<RepeatingKeyXorCipher> {
        get_probable_key_sizes(encrypted)
            .iter()
            .filter_map(|&keysize| self.decrypt_with_keysize(encrypted, keysize))
            .max_by(|a, b| a.score.total_cmp(&b.score))
    }

    fn decrypt_with_keysize(
        self: &Self,
        encrypted: &[u8],
        keysize: usize,
    ) -> Option<RepeatingKeyXorCipher> {
        // combine all of the bytes into a single string
        let mut key: Vec<u8> = Vec::new();
        for slice in get_vertical_slices(encrypted, keysize) {
            // find the most likely solution for each vertical slice
            let cipher = self.single_byte_xor_decryptor.decrypt(&slice)?;
            key.push(cipher.key);
        }

        let cleartext_bytes = xor_with_key(encrypted.iter().cloned(), &key).collect();
        let cleartext = String::from_utf8(cleartext_bytes).ok()?;
        let score = self.standard_freq.score_str(&cleartext);

        Some(RepeatingKeyXorCipher { key, score })
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn get_vertical_slices_should_work() {
        let input = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let row_length = 4;
        let expected = vec![vec![0, 4, 8], vec![1, 5, 9], vec![2, 6], vec![3, 7]];

        let actual = get_vertical_slices(&input, row_length);
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_chunk_should_work() {
        let input = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let cases = vec![
            (0, 4, vec![0, 1, 2, 3]),
            (1, 4, vec![4, 5, 6, 7]),
            (2, 4, vec![8, 9]),
            (3, 4, vec![]),
        ];

        for (num, size, expected) in cases {
            let actual = get_chunk(&input, size, num);
            assert_eq!(expected, actual);
        }
    }
}
