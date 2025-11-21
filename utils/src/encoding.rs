use base64::{Engine, engine::general_purpose::STANDARD_NO_PAD};
use hex;

pub fn hex_encode(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

pub fn hex_decode(s: &str) -> Vec<u8> {
    hex::decode(s).expect("unable to decode")
}

pub fn base64_encode(bytes: &[u8]) -> String {
    STANDARD_NO_PAD.encode(bytes)
}

pub fn base64_decode(bytes: &[u8]) -> Vec<u8> {
    STANDARD_NO_PAD.decode(bytes).expect("unable to decode")
}
