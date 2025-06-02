pub fn hex_str(bytes: &[u8]) -> String {
    bytes.iter()
         .map(|b| format!("{:x}", b))
         .collect()
}