use std::iter::repeat;

pub mod analysis;
pub mod ciphers;
pub mod encoding;
pub mod frequency;

pub fn xor_buffers(buffer1: &[u8], buffer2: &[u8]) -> Vec<u8> {
    buffer1
        .into_iter()
        .zip(buffer2.into_iter())
        .map(|(a, b)| a ^ b)
        .collect()
}

pub fn xor_with_byte(stream: impl Iterator<Item = u8>, key: u8) -> impl Iterator<Item = u8> {
    stream.zip(repeat(key)).map(|(a, b)| a ^ b)
}

pub fn xor_with_key(stream: impl Iterator<Item = u8>, key: &[u8]) -> impl Iterator<Item = u8> {
    stream
        .zip(repeat(key).flatten().copied())
        .map(|(a, b)| a ^ b)
}

pub fn xor_streams(
    stream1: impl Iterator<Item = u8>,
    stream2: impl Iterator<Item = u8>,
) -> impl Iterator<Item = u8> {
    stream1.zip(stream2.into_iter()).map(|(a, b)| a ^ b)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn xor_with_byte_should_work() {
        let input = vec![0, 1, 2, 3];
        let key = 2;
        let expected = vec![2, 3, 0, 1];

        let actual: Vec<_> = xor_with_byte(input.into_iter(), key).collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn xor_with_key_should_work() {
        let input1 = vec![0, 1, 2, 3, 0, 1, 2, 3];
        let key = vec![4, 2, 1];
        let expected = vec![4, 3, 3, 7, 2, 0, 6, 1];

        let actual: Vec<_> = xor_with_key(input1.iter().copied(), &key).collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn xor_streams_should_work() {
        let input1 = vec![0, 1, 2, 3];
        let input2 = vec![8, 4, 2, 1];
        let expected = vec![8, 5, 0, 2];

        let actual: Vec<_> = xor_streams(input1.iter().copied(), input2.iter().copied()).collect();

        assert_eq!(expected, actual);
    }
}
