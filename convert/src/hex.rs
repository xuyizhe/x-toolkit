pub fn encode<I: AsRef<[u8]>>(input: I) -> String {
    hex::encode(input)
}

pub fn decode<I: AsRef<[u8]>>(input: I) -> Vec<u8> {
    hex::decode(input).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex() {
        let expected = [
            9, 202, 126, 78, 170, 110, 138, 233, 199, 210, 97, 22, 113, 41, 24, 72, 131, 100, 77,
            7, 223, 186, 124, 191, 188, 76, 138, 46, 8, 54, 13, 91,
        ];
        let encoded = hex::encode(expected);
        assert_eq!(
            encoded,
            String::from("09ca7e4eaa6e8ae9c7d261167129184883644d07dfba7cbfbc4c8a2e08360d5b")
        );
        assert_eq!(decode(encoded), expected);
    }
}
