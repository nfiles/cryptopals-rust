extern crate utils;

use std::fs;
use utils::{
    ciphers::{frequency::CharSet, single_byte_xor::SingleByteXorDecryptor},
    encoding::{base64_encode, hex_decode, hex_encode},
    xor_buffers,
};

const CORPUS: &str = "assets/pg19033-alice-in-wonderland.txt";

#[test]
fn problem01_convert_hex_to_base64() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    let decoded = hex_decode(&input);
    let actual = base64_encode(&decoded);

    assert_eq!(expected, actual);
}

#[test]
fn problem02_fixed_xor() {
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
fn problem03_single_byte_xor() {
    let corpus_text = fs::read_to_string(CORPUS).expect("unable to read file");
    let decryptor = SingleByteXorDecryptor::from_corpus(CharSet::default(), &corpus_text);

    let encoded =
        hex_decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    let result = decryptor.decrypt(&encoded).expect("no solution");

    assert_eq!(88u8, result.key);
    assert_eq!("Cooking MC's like a pound of bacon", result.cleartext);
}
