use base64::{Engine, prelude::BASE64_STANDARD};
use hex;

pub fn hex_encode(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

pub fn hex_decode(s: &str) -> Vec<u8> {
    hex::decode(s).expect("unable to decode")
}

pub fn base64_encode(bytes: &[u8]) -> String {
    BASE64_STANDARD.encode(&bytes)
}

pub fn base64_decode(bytes: &[u8]) -> Vec<u8> {
    let stripped: Vec<u8> = bytes
        .into_iter()
        .cloned()
        .filter(|&x| x != b'\r' && x != b'\n')
        .collect();

    BASE64_STANDARD.decode(stripped).expect("unable to decode")
}
