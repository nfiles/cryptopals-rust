pub fn get_hamming_distance(left: &[u8], right: &[u8]) -> usize {
    let mut distance = left.len().abs_diff(right.len()) * 8;

    let length = if left.len() < right.len() {
        left.len()
    } else {
        right.len()
    };

    for i in 0..length {
        let diff = left[i] ^ right[i];
        distance += (0..8).map(|p| diff & (1 << p) > 0).filter(|x| *x).count();
    }

    distance
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn get_hamming_distance_should_work() {
        let left = b"this is a test";
        let right = b"wokka wokka!!!";

        let distance = get_hamming_distance(left, right);

        assert_eq!(37, distance);
    }
}
