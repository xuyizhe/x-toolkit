use ring::digest;
use std::convert::TryInto;

pub fn sha256(bytes: &[u8]) -> Vec<u8> {
    digest::digest(&digest::SHA256, bytes)
        .as_ref()
        .try_into()
        .unwrap()
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
            vec![
                9, 202, 126, 78, 170, 110, 138, 233, 199, 210, 97, 22, 113, 41, 24, 72, 131, 100,
                77, 7, 223, 186, 124, 191, 188, 76, 138, 46, 8, 54, 13, 91
            ]
        );
        assert_eq!(sha256(b"hello, world"), expected);
    }
}
