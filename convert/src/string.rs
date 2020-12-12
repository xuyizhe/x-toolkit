pub fn from_bytes(data: &[u8]) -> String {
    std::str::from_utf8(data).unwrap().to_string()
}

pub fn to_bytes(str: String) -> Vec<u8> {
    str.into_bytes()
}
