pub trait ToHex {
    fn to_hex_string(&self) -> String;
}

impl ToHex for &[u8] {
    fn to_hex_string(&self) -> String {
        self.iter().map(|b| format!("{:02x}", b)).collect()
    }
}
