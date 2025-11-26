use std::collections::{HashMap, HashSet};

pub type FrequencyMap = HashMap<char, f64>;

#[derive(Clone)]
pub struct Frequency {
    map: FrequencyMap,
}

impl Frequency {
    pub fn from_corpus(corpus: &str) -> Self {
        let mut totals: HashMap<char, u32> = HashMap::new();
        for ch in corpus.chars() {
            totals
                .entry(ch)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let grand_total: u32 = totals.values().sum();

        if grand_total == 0 {
            return Frequency {
                map: HashMap::new(),
            };
        }

        let map: FrequencyMap = totals
            .iter()
            .map(|(&ch, &total)| (ch, (f64::from(total) / f64::from(grand_total))))
            .collect();

        Frequency { map }
    }

    pub fn compare_with(self: &Self, right: &Self) -> f64 {
        let left_keys = self.map.keys();
        let right_keys = right.map.keys();

        let alphabet: HashSet<_> = left_keys.chain(right_keys).into_iter().collect();
        alphabet
            .iter()
            .map(|ch| {
                let freq_left = self.map.get(ch).copied().unwrap_or_default();
                let freq_right = right.map.get(ch).copied().unwrap_or_default();
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

        for (input, expected) in cases {
            let actual = Frequency::from_corpus(&input);
            assert_eq!(actual.map, expected);
        }
    }
}
