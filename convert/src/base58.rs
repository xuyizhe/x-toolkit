pub fn encode<I: AsRef<[u8]>>(input: I) -> String {
    bs58::encode(input)
        .with_alphabet(bs58::Alphabet::BITCOIN)
        .into_string()
}

pub fn decode<I: AsRef<[u8]>>(input: I) -> Vec<u8> {
    bs58::decode(input)
        .with_alphabet(bs58::Alphabet::BITCOIN)
        .into_vec()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base58() {
        let expected = vec![
            9, 202, 126, 78, 170, 110, 138, 233, 199, 210, 97, 22, 113, 41, 24, 72, 131, 100, 77,
            7, 223, 186, 124, 191, 188, 76, 138, 46, 8, 54, 13, 91,
        ];
        let encoded = encode(&expected);
        assert_eq!(
            encoded,
            String::from("fDkpN2xbAwr2sKNveZKDJkamzCiKXNqgVrMbfBR6KHk"),
        );
        assert_eq!(decode(encoded), expected);
    }
}
