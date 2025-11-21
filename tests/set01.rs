extern crate utils;

use utils::{
    encoding::{base64_encode, hex_decode, hex_encode},
    xor_buffers,
};

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
    let xord = xor_buffers(&bytes1, &bytes2).expect("unable to xor the buffers");
    let actual = hex_encode(&xord);

    assert_eq!(expected, actual);
}

#[test]
#[ignore = "not implemented"]
fn problem03_single_byte_xor() {}
