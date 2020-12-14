use crate::ripemd160::{Digest, Ripemd160};
use ring::digest;
use std::convert::TryInto;

pub fn sha256(data: &[u8]) -> [u8; 32] {
    digest::digest(&digest::SHA256, data)
        .as_ref()
        .try_into()
        .unwrap()
}

pub fn ripemd160(data: &[u8]) -> [u8; 20] {
    let mut hasher = Ripemd160::new();
    hasher.update(data);
    let result = hasher.finalize();
    result[..].try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256() {
        let expected_hex = "09ca7e4eaa6e8ae9c7d261167129184883644d07dfba7cbfbc4c8a2e08360d5b";
        let expected: Vec<u8> = ring::test::from_hex(expected_hex).unwrap();
        assert_eq!(
            sha256(b"hello, world"),
            [
                9, 202, 126, 78, 170, 110, 138, 233, 199, 210, 97, 22, 113, 41, 24, 72, 131, 100,
                77, 7, 223, 186, 124, 191, 188, 76, 138, 46, 8, 54, 13, 91
            ]
        );
        assert_eq!(sha256(b"hello, world"), expected[..]);
    }

    #[test]
    fn test_ripmd160() {
        assert_eq!(
            ripemd160(b"hello, world"),
            [
                163, 32, 31, 130, 252, 160, 52, 228, 109, 16, 205, 123, 39, 225, 116, 151, 110, 36,
                29, 162
            ]
        );
    }
}
