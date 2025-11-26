use std::iter::repeat_with;

use crate::{
    analysis::get_hamming_distance,
    ciphers::{
        frequency::Frequency,
        single_byte_xor::{SingleByteXorCipher, SingleByteXorDecryptor},
    },
};

pub fn get_probable_key_sizes(encrypted: &[u8]) -> Vec<usize> {
    if encrypted.len() == 1 {
        return vec![1];
    }

    let mut possible_key_sizes: Vec<_> = (2..40)
        // only consider keys that are shorter than the ciphertext and are even multiples
        .filter(|&keysize| keysize < encrypted.len())
        .filter_map(|keysize| {
            let chunk1 = get_chunk(encrypted, keysize, 0);
            let chunk2 = get_chunk(encrypted, keysize, 1);

            let distance = get_hamming_distance(&chunk1, &chunk2);
            let normalized = distance as f64 / keysize as f64;

            Some((normalized, keysize))
        })
        .collect();

    possible_key_sizes.sort_by(|(left, _), (right, _)| left.total_cmp(&right));

    let num_possible_key_sizes = possible_key_sizes.len() / 5;
    let num_possible_key_sizes = match num_possible_key_sizes {
        0 => 1,
        _ => num_possible_key_sizes,
    };

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
}

impl RepeatingKeyXorDecryptor {
    pub fn from_corpus(corpus: &str) -> RepeatingKeyXorDecryptor {
        let standard_freq = Frequency::from_corpus(&corpus);
        RepeatingKeyXorDecryptor { standard_freq }
    }

    pub fn decrypt(self: &Self, encrypted: &[u8]) -> Option<RepeatingKeyXorCipher> {
        let probable_key_sizes = get_probable_key_sizes(encrypted);

        let solutions: Vec<_> = probable_key_sizes
            .iter()
            .cloned()
            .filter_map(|keysize| self.decrypt_with_keysize(encrypted, keysize))
            .collect();

        solutions
            .iter()
            .min_by(|a, b| a.score.total_cmp(&b.score))
            .cloned()
    }

    fn decrypt_with_keysize(
        self: &Self,
        encrypted: &[u8],
        keysize: usize,
    ) -> Option<RepeatingKeyXorCipher> {
        let single_byte_xor_decryptor =
            SingleByteXorDecryptor::with_standard_freq(self.standard_freq.clone());

        let mut solution_bytes: Vec<SingleByteXorCipher> = Vec::new();
        for slice in get_vertical_slices(encrypted, keysize) {
            let solution = single_byte_xor_decryptor.decrypt(&slice)?;
            solution_bytes.push(solution);
        }

        let score =
            solution_bytes.iter().map(|c| &c.score).sum::<f64>() / (solution_bytes.len() as f64);
        let key: Vec<_> = solution_bytes.iter().map(|s| s.key).collect();

        Some(RepeatingKeyXorCipher { score, key })
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
