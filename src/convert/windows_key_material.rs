use std::collections::BTreeMap;
use std::fmt;

use crate::domain::BluetoothAddress;
use crate::error::{BluebondError, Result};

#[derive(Clone, Eq, PartialEq)]
pub struct WindowsBluetoothKeyMaterial {
    pub ltk: Option<Vec<u8>>,
    pub irk: Option<Vec<u8>>,
    pub csrk: Option<Vec<u8>>,
    pub erand: Option<Vec<u8>>,
    pub ediv: Option<u32>,
    pub address: Option<BluetoothAddress>,
    pub address_type: Option<u32>,
    pub key_length: Option<u32>,
    pub auth_req: Option<u32>,
}

impl fmt::Debug for WindowsBluetoothKeyMaterial {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WindowsBluetoothKeyMaterial")
            .field("ltk", &redacted_optional_bytes(&self.ltk))
            .field("irk", &redacted_optional_bytes(&self.irk))
            .field("csrk", &redacted_optional_bytes(&self.csrk))
            .field("erand", &redacted_optional_bytes(&self.erand))
            .field("ediv", &self.ediv)
            .field("address", &self.address)
            .field("address_type", &self.address_type)
            .field("key_length", &self.key_length)
            .field("auth_req", &self.auth_req)
            .finish()
    }
}

impl WindowsBluetoothKeyMaterial {
    pub fn has_core_key_material(&self) -> bool {
        self.ltk.as_ref().is_some_and(|value| !value.is_empty())
            || self.irk.as_ref().is_some_and(|value| !value.is_empty())
            || self.csrk.as_ref().is_some_and(|value| !value.is_empty())
    }
}

fn redacted_optional_bytes(value: &Option<Vec<u8>>) -> &'static str {
    if value.is_some() {
        "<redacted>"
    } else {
        "None"
    }
}

pub fn parse_hivexsh_lsval_output(output: &str) -> Result<WindowsBluetoothKeyMaterial> {
    let values = parse_named_values(output);

    Ok(WindowsBluetoothKeyMaterial {
        ltk: parse_optional_hex_bytes(&values, "LTK")?,
        irk: parse_optional_hex_bytes(&values, "IRK")?,
        csrk: parse_optional_hex_bytes(&values, "CSRK")?,
        erand: parse_optional_hex_bytes(&values, "ERand")?,
        ediv: parse_optional_dword(&values, "EDIV")?,
        address: parse_optional_windows_address(&values)?,
        address_type: parse_optional_dword(&values, "AddressType")?,
        key_length: parse_optional_dword(&values, "KeyLength")?,
        auth_req: parse_optional_dword(&values, "AuthReq")?,
    })
}

fn parse_named_values(output: &str) -> BTreeMap<String, String> {
    output
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            let (name, value) = trimmed.split_once('=')?;
            let name = name.trim().trim_matches('"');
            if name.is_empty() {
                return None;
            }

            Some((name.to_string(), value.trim().to_string()))
        })
        .collect()
}

fn parse_optional_hex_bytes(
    values: &BTreeMap<String, String>,
    name: &str,
) -> Result<Option<Vec<u8>>> {
    values
        .get(name)
        .map(|value| parse_hex_value_bytes(value))
        .transpose()
}

fn parse_optional_dword(values: &BTreeMap<String, String>, name: &str) -> Result<Option<u32>> {
    values.get(name).map(|value| parse_dword(value)).transpose()
}

fn parse_optional_windows_address(
    values: &BTreeMap<String, String>,
) -> Result<Option<BluetoothAddress>> {
    let Some(bytes) = parse_optional_hex_bytes(values, "Address")? else {
        return Ok(None);
    };

    if bytes.len() < 6 {
        return Err(invalid_address("Address"));
    }

    let compact = bytes[..6]
        .iter()
        .rev()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();

    BluetoothAddress::from_compact_hex(&compact)
        .map(Some)
        .map_err(|_| invalid_address("Address"))
}

fn parse_hex_value_bytes(value: &str) -> Result<Vec<u8>> {
    let Some((kind, bytes)) = value.split_once(':') else {
        return Err(invalid_value("hex value"));
    };

    if !kind.starts_with("hex(") {
        return Err(invalid_value("hex value"));
    }

    bytes
        .split(',')
        .map(str::trim)
        .filter(|chunk| !chunk.is_empty())
        .map(|chunk| u8::from_str_radix(chunk, 16).map_err(|_| invalid_value("hex byte")))
        .collect()
}

fn parse_dword(value: &str) -> Result<u32> {
    let Some(hex) = value.strip_prefix("dword:") else {
        return Err(invalid_value("dword"));
    };

    u32::from_str_radix(hex.trim(), 16).map_err(|_| invalid_value("dword"))
}

fn invalid_value(context: &'static str) -> BluebondError {
    BluebondError::InvalidRegistryValue { context }
}

fn invalid_address(context: &'static str) -> BluebondError {
    BluebondError::InvalidRegistryValue { context }
}
