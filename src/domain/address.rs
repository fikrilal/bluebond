use std::fmt;
use std::str::FromStr;

use crate::error::{BluebondError, Result};

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BluetoothAddress {
    bytes: [u8; 6],
}

impl BluetoothAddress {
    pub fn bytes(self) -> [u8; 6] {
        self.bytes
    }

    pub fn compact_lower(self) -> String {
        self.bytes
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect()
    }
}

impl FromStr for BluetoothAddress {
    type Err = BluebondError;

    fn from_str(input: &str) -> Result<Self> {
        let normalized = input.trim();
        let parts: Vec<&str> = normalized.split(':').collect();
        if parts.len() != 6 {
            return Err(BluebondError::InvalidBluetoothAddress {
                input: input.to_string(),
            });
        }

        let mut bytes = [0_u8; 6];
        for (index, part) in parts.iter().enumerate() {
            if part.len() != 2 {
                return Err(BluebondError::InvalidBluetoothAddress {
                    input: input.to_string(),
                });
            }

            bytes[index] = u8::from_str_radix(part, 16).map_err(|_| {
                BluebondError::InvalidBluetoothAddress {
                    input: input.to_string(),
                }
            })?;
        }

        Ok(Self { bytes })
    }
}

impl fmt::Display for BluetoothAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, byte) in self.bytes.iter().enumerate() {
            if index > 0 {
                f.write_str(":")?;
            }
            write!(f, "{byte:02X}")?;
        }
        Ok(())
    }
}

impl fmt::Debug for BluetoothAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_colon_separated_address() {
        let address: BluetoothAddress = "f8:89:d2:83:92:c0".parse().unwrap();

        assert_eq!(address.to_string(), "F8:89:D2:83:92:C0");
        assert_eq!(address.compact_lower(), "f889d28392c0");
        assert_eq!(address.bytes(), [0xF8, 0x89, 0xD2, 0x83, 0x92, 0xC0]);
    }

    #[test]
    fn rejects_invalid_address() {
        let result = "f8:89:d2".parse::<BluetoothAddress>();

        assert!(result.is_err());
    }
}
