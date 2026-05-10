pub fn hex_upper(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02X}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_uppercase_hex() {
        assert_eq!(hex_upper(&[0x85, 0x35, 0xda]), "8535DA");
    }
}
