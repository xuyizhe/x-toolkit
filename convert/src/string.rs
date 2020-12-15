pub fn from_bytes(input: &[u8]) -> String {
    std::str::from_utf8(input).unwrap().to_string()
}

pub fn to_bytes(input: String) -> Vec<u8> {
    input.into_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let bytes = to_bytes(String::from("hello, world"));
        assert_eq!(bytes, b"hello, world");
        assert_eq!(from_bytes(&bytes[..]), String::from("hello, world"));
    }
}
