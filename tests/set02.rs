extern crate utils;

use std::iter;
use test_case::test_case;
use utils::ciphers::aes::Pkcs7Padding;

#[test_case(20, 4;  "when blocksize 20 should have 4 bytes of padding")]
#[test_case(16, 16; "when blocksize 16 should have 16 bytes of padding")]
#[test_case(10, 4;  "when blocksize 16 should have 16 bytes of padding")]
#[test_case(5, 4;   "when blocksize 16 should have 16 bytes of padding")]
fn challenge09_pkcs7_padding(blocksize: usize, padding_byte: u8) {
    let input = b"YELLOW SUBMARINE";
    let padding = iter::repeat(padding_byte).take(padding_byte as usize);
    let expected: Vec<_> = input.iter().cloned().chain(padding).collect();

    let padding = Pkcs7Padding::create(blocksize).unwrap();
    let actual = padding.pad(input);

    assert_eq!(expected, actual);
}

#[test_case(true,  vec![4u8;   4]; "blocksize 4 with 4 bytes of padding is right")]
#[test_case(true,  vec![16u8; 16]; "blocksize 16 with 16 bytes of padding is right")]
#[test_case(false, vec![4u8;   3]; "blocksize 4 with 3 bytes of padding is wrong")]
#[test_case(false, vec![0u8;   0]; "blocksize 0 with 0 bytes of padding is wrong")]
fn challenge09_pkcs7_unpadding(success: bool, padding: Vec<u8>) {
    let expected = b"YELLOW SUBMARINE";
    let input: Vec<_> = b"YELLOW SUBMARINE"
        .iter()
        .cloned()
        .chain(padding)
        .collect();

    let pkcs7_padding = Pkcs7Padding::create(4).unwrap();
    let result = pkcs7_padding.unpad(&input);

    if success {
        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    } else {
        assert!(result.is_err())
    }
}
