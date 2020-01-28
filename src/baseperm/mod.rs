use base32;
use base64;

// NOTE(ww): It'd be really nice if the base* modules used here exposed
// their alphabets, but it's not a big deal: they won't be changing anytime
// soon.

pub trait DecodeContext {
    fn alphabet(&self) -> &'static [u8];
    fn bitness(&self) -> usize;
    fn is_valid_byte(&self, b: u8) -> bool;
    fn decode(&self, input: &str) -> Option<Vec<u8>>;
}

pub struct Base64;
impl DecodeContext for Base64 {
    fn alphabet(&self) -> &'static [u8] {
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
    }

    fn bitness(&self) -> usize {
        6
    }

    fn is_valid_byte(&self, b: u8) -> bool {
        self.alphabet().iter().any(|&a| a == b)
    }

    fn decode(&self, input: &str) -> Option<Vec<u8>> {
        match base64::decode(input) {
            Ok(result) => Some(result),
            Err(_) => None,
        }
    }
}

pub struct Base64Urlsafe;
impl DecodeContext for Base64Urlsafe {
    fn alphabet(&self) -> &'static [u8] {
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_"
    }

    fn bitness(&self) -> usize {
        6
    }

    fn is_valid_byte(&self, b: u8) -> bool {
        self.alphabet().iter().any(|&a| a == b)
    }

    fn decode(&self, input: &str) -> Option<Vec<u8>> {
        match base64::decode_config(input, base64::URL_SAFE) {
            Ok(result) => Some(result),
            Err(_) => None,
        }
    }
}

pub struct Base32;
impl DecodeContext for Base32 {
    fn alphabet(&self) -> &'static [u8] {
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"
    }

    fn bitness(&self) -> usize {
        5
    }

    fn is_valid_byte(&self, b: u8) -> bool {
        self.alphabet().iter().any(|&a| a == b)
    }

    fn decode(&self, input: &str) -> Option<Vec<u8>> {
        base32::decode(base32::Alphabet::RFC4648 { padding: true }, input)
    }
}
