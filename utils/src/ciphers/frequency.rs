use std::collections::{HashMap, HashSet};

pub type FrequencyMap = HashMap<char, f64>;

pub struct CharSet {
    alphabet: HashSet<char>,
    transform: fn(ch: char) -> char,
}

impl CharSet {
    pub fn default() -> Self {
        CharSet {
            alphabet: ('a'..='z').collect(),
            transform: |ch| ch.to_ascii_lowercase(),
        }
    }

    pub fn new(alphabet: HashSet<char>) -> Self {
        CharSet {
            alphabet,
            transform: |ch| ch.to_ascii_lowercase(),
        }
    }

    pub fn build_freq_from_corpus(self: &Self, corpus: &str) -> FrequencyMap {
        let mut totals: HashMap<char, u32> = HashMap::new();
        for ch in corpus
            .chars()
            .map(|ch| (self.transform)(ch))
            .filter(|&ch| self.alphabet.contains(&ch))
        {
            totals
                .entry(ch)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let grand_total: u32 = totals.values().sum();

        if grand_total == 0 {
            return HashMap::new();
        }

        totals
            .iter()
            .map(|(&ch, &total)| (ch, (f64::from(total) / f64::from(grand_total))))
            .collect()
    }

    pub fn compare_freq(self: &Self, left: &FrequencyMap, right: &FrequencyMap) -> f64 {
        self.alphabet
            .iter()
            .map(|ch| {
                let freq_left = left.get(ch).copied().unwrap_or_default();
                let freq_right = right.get(ch).copied().unwrap_or_default();
                (freq_left - freq_right).abs()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_expected_frequecies() {
        let cases = vec![
            ("", HashMap::new()),
            (
                "abcd",
                HashMap::from([('a', 0.25), ('b', 0.25), ('c', 0.25), ('d', 0.25)]),
            ),
            (
                "ABCD",
                HashMap::from([('a', 0.25), ('b', 0.25), ('c', 0.25), ('d', 0.25)]),
            ),
            (
                "Hello, you!",
                HashMap::from([
                    ('h', 0.125),
                    ('e', 0.125),
                    ('l', 0.25),
                    ('o', 0.25),
                    ('y', 0.125),
                    ('u', 0.125),
                ]),
            ),
        ];

        let charset = CharSet::default();

        for (input, expected) in cases {
            let actual = charset.build_freq_from_corpus(&input);
            assert_eq!(actual, expected);
        }
    }
}
