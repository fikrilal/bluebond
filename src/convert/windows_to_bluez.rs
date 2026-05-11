use std::fmt;

use crate::convert::endian;
use crate::convert::windows_key_material::WindowsBluetoothKeyMaterial;
use crate::error::{BluebondError, Result};

const AUTH_REQ_MITM_BIT: u32 = 0x04;

#[derive(Clone, Eq, PartialEq)]
pub struct BluezKeyHex(String);

impl BluezKeyHex {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(hex_upper(bytes))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for BluezKeyHex {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("<redacted>")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BluezLongTermKeySection {
    pub key: BluezKeyHex,
    pub authenticated: bool,
    pub enc_size: u32,
    pub ediv: u32,
    pub rand: u64,
}

impl BluezLongTermKeySection {
    fn render_group(&self, group: &str, output: &mut String) {
        push_group(output, group);
        push_key_value(output, "Key", self.key.as_str());
        push_key_value(output, "Authenticated", bool_value(self.authenticated));
        push_key_value(output, "EncSize", &self.enc_size.to_string());
        push_key_value(output, "EDiv", &self.ediv.to_string());
        push_key_value(output, "Rand", &self.rand.to_string());
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BluezIdentityResolvingKeySection {
    pub key: BluezKeyHex,
}

impl BluezIdentityResolvingKeySection {
    fn render(&self, output: &mut String) {
        push_group(output, "IdentityResolvingKey");
        push_key_value(output, "Key", self.key.as_str());
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BluezSignatureKeySection {
    pub key: BluezKeyHex,
    pub counter: u32,
    pub authenticated: bool,
}

impl BluezSignatureKeySection {
    fn render_group(&self, group: &str, output: &mut String) {
        push_group(output, group);
        push_key_value(output, "Key", self.key.as_str());
        push_key_value(output, "Counter", &self.counter.to_string());
        push_key_value(output, "Authenticated", bool_value(self.authenticated));
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BluezInfoKeySections {
    pub identity_resolving_key: Option<BluezIdentityResolvingKeySection>,
    pub local_signature_key: Option<BluezSignatureKeySection>,
    pub long_term_key: Option<BluezLongTermKeySection>,
}

impl BluezInfoKeySections {
    pub fn from_windows_key_material(material: &WindowsBluetoothKeyMaterial) -> Result<Self> {
        Ok(Self {
            identity_resolving_key: material.irk.as_deref().map(|key| {
                BluezIdentityResolvingKeySection {
                    key: BluezKeyHex::from_bytes(key),
                }
            }),
            local_signature_key: material
                .csrk
                .as_deref()
                .map(|key| BluezSignatureKeySection {
                    key: BluezKeyHex::from_bytes(key),
                    counter: 0,
                    authenticated: authenticated_from_auth_req(material.auth_req),
                }),
            long_term_key: build_long_term_key(material)?,
        })
    }

    pub fn render(&self) -> String {
        let mut output = String::new();

        if let Some(identity_resolving_key) = &self.identity_resolving_key {
            identity_resolving_key.render(&mut output);
        }

        if let Some(local_signature_key) = &self.local_signature_key {
            local_signature_key.render_group("LocalSignatureKey", &mut output);
        }

        if let Some(long_term_key) = &self.long_term_key {
            long_term_key.render_group("LongTermKey", &mut output);
        }

        output
    }
}

pub fn hex_upper(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02X}")).collect()
}

fn build_long_term_key(
    material: &WindowsBluetoothKeyMaterial,
) -> Result<Option<BluezLongTermKeySection>> {
    let Some(key) = material.ltk.as_deref() else {
        return Ok(None);
    };

    let ediv = material
        .ediv
        .ok_or(BluebondError::InvalidRegistryValue { context: "EDIV" })?;
    let erand = material
        .erand
        .as_deref()
        .ok_or(BluebondError::InvalidRegistryValue { context: "ERand" })?;

    if erand.len() != 8 {
        return Err(BluebondError::InvalidRegistryValue { context: "ERand" });
    }

    let erand: [u8; 8] = erand
        .try_into()
        .map_err(|_| BluebondError::InvalidRegistryValue { context: "ERand" })?;

    Ok(Some(BluezLongTermKeySection {
        key: BluezKeyHex::from_bytes(key),
        authenticated: authenticated_from_auth_req(material.auth_req),
        enc_size: material.key_length.unwrap_or(key.len() as u32),
        ediv,
        rand: endian::u64_from_le_bytes(erand),
    }))
}

fn authenticated_from_auth_req(auth_req: Option<u32>) -> bool {
    auth_req.is_some_and(|value| value & AUTH_REQ_MITM_BIT != 0)
}

fn push_group(output: &mut String, group: &str) {
    if !output.is_empty() {
        output.push('\n');
    }

    output.push('[');
    output.push_str(group);
    output.push_str("]\n");
}

fn push_key_value(output: &mut String, key: &str, value: &str) {
    output.push_str(key);
    output.push('=');
    output.push_str(value);
    output.push('\n');
}

fn bool_value(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_uppercase_hex() {
        assert_eq!(hex_upper(&[0x85, 0x35, 0xda]), "8535DA");
    }
}
