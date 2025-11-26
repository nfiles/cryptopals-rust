extern crate utils;

use std::{
    fs::{self, File},
    io::{self, BufRead},
};
use utils::{
    ciphers::single_byte_xor::SingleByteXorDecryptor,
    encoding::{base64_encode, hex_decode, hex_encode},
    xor_buffers, xor_with_key,
};

const CORPUS: &str = "assets/pg19033-alice-in-wonderland.txt";

#[test]
fn challenge01_convert_hex_to_base64() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    let decoded = hex_decode(&input);
    let actual = base64_encode(&decoded);

    assert_eq!(expected, actual);
}

#[test]
fn challenge02_fixed_xor() {
    let input1 = "1c0111001f010100061a024b53535009181c";
    let input2 = "686974207468652062756c6c277320657965";
    let expected = "746865206b696420646f6e277420706c6179";

    let bytes1 = hex_decode(input1);
    let bytes2 = hex_decode(input2);
    let xord = xor_buffers(&bytes1, &bytes2);
    let actual = hex_encode(&xord);

    assert_eq!(expected, actual);
}

#[test]
fn challenge03_single_byte_xor() {
    let corpus_text = fs::read_to_string(CORPUS).expect("unable to read file");
    let decryptor = SingleByteXorDecryptor::from_corpus(&corpus_text);

    let encoded =
        hex_decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    let result = decryptor.decrypt(&encoded).expect("no solution");

    assert_eq!(88u8, result.key);
    assert_eq!("Cooking MC's like a pound of bacon", result.cleartext);
}

#[test]
fn challenge04_detect_single_byte_xor() {
    let corpus_text = fs::read_to_string(CORPUS).expect("unable to read file");
    let decryptor = SingleByteXorDecryptor::from_corpus(&corpus_text);

    let input_file = File::open("assets/set01-4.txt").expect("unable to open file");
    let input_lines = io::BufReader::new(input_file).lines().map_while(Result::ok);

    let options: Vec<_> = input_lines
        .filter_map(|encoded| {
            let bytes = hex_decode(&encoded);
            match decryptor.decrypt(&bytes) {
                Some(cipher) => Some(cipher),
                None => None,
            }
        })
        .collect();

    let result = options
        .iter()
        .min_by(|cipher1, cipher2| f64::total_cmp(&cipher1.score, &cipher2.score))
        .expect("unable to decrypt any lines");

    assert_eq!(53, result.key);
    assert_eq!("Now that the party is jumping\n", result.cleartext);
}

#[test]
fn challenge05_repeating_key_xor() {
    let input = b"Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
    let key = b"ICE";
    let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272
a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        .replace("\n", "");

    let actual = hex_encode(&xor_with_key(input.iter().cloned(), key).collect::<Vec<_>>());

    assert_eq!(actual, expected);
}
