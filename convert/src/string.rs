pub fn from_bytes(data: &[u8]) -> String {
    std::str::from_utf8(data).unwrap().to_string()
}

pub fn to_bytes(data: String) -> Vec<u8> {
    data.into_bytes()
}
