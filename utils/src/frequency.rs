use std::collections::HashMap;

pub type FrequencyMap = HashMap<char, f64>;

#[derive(Clone)]
pub struct Frequency {
    map: FrequencyMap,
}

impl Frequency {
    pub fn analyze(corpus: &str) -> Self {
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

    pub fn score_str(self: &Self, text: &str) -> f64 {
        let chars: Vec<char> = text.chars().collect();
        self.score(&chars)
    }

    pub fn score(self: &Self, stream: &[char]) -> f64 {
        let total_chars = stream.len() as f64;

        stream
            .iter()
            .cloned()
            .map(|ch| self.map.get(&ch).copied().unwrap_or_default())
            .sum::<f64>()
            / total_chars
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
                HashMap::from([('A', 0.25), ('B', 0.25), ('C', 0.25), ('D', 0.25)]),
            ),
            (
                "ABCDabcd",
                HashMap::from([
                    ('A', 0.125),
                    ('B', 0.125),
                    ('C', 0.125),
                    ('D', 0.125),
                    ('a', 0.125),
                    ('b', 0.125),
                    ('c', 0.125),
                    ('d', 0.125),
                ]),
            ),
            (
                "Hello you!",
                HashMap::from([
                    ('H', 0.1),
                    ('e', 0.1),
                    ('l', 0.2),
                    ('o', 0.2),
                    (' ', 0.1),
                    ('y', 0.1),
                    ('u', 0.1),
                    ('!', 0.1),
                ]),
            ),
        ];

        for (input, expected) in cases {
            let actual = Frequency::analyze(&input);
            assert_eq!(actual.map, expected);
        }
    }
}
