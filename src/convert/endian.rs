pub fn u64_from_le_bytes(bytes: [u8; 8]) -> u64 {
    u64::from_le_bytes(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_little_endian_bytes() {
        let value = u64_from_le_bytes([0x50, 0xC4, 0xCC, 0x03, 0x79, 0x3B, 0xDC, 0x05]);

        assert_eq!(value, 422_277_856_006_816_848);
    }
}
