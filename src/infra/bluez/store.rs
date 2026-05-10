use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::domain::{BluetoothAdapter, BluetoothAddress, BluetoothDevice};
use crate::error::{BluebondError, Result};
use tracing::debug;

pub const DEFAULT_BLUEZ_DIR: &str = "/var/lib/bluetooth";

pub fn default_store_exists() -> bool {
    Path::new(DEFAULT_BLUEZ_DIR).exists()
}

pub fn read_inventory(bluez_dir: &Path) -> Result<Vec<BluetoothAdapter>> {
    let entries = fs::read_dir(bluez_dir).map_err(|source| BluebondError::Io {
        context: "reading BlueZ store directory",
        source,
    })?;

    let mut adapters = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|source| BluebondError::Io {
            context: "reading BlueZ store entry",
            source,
        })?;
        let file_type = entry.file_type().map_err(|source| BluebondError::Io {
            context: "reading BlueZ store entry type",
            source,
        })?;

        if !file_type.is_dir() {
            continue;
        }

        let Some(adapter_address) = parse_dir_address(&entry.file_name()) else {
            debug!(
                path = %entry.path().display(),
                "skipping non-address BlueZ store entry"
            );
            continue;
        };

        let devices = read_adapter_devices(&entry.path())?;
        adapters.push(BluetoothAdapter {
            address: adapter_address,
            devices,
        });
    }

    adapters.sort_by_key(|adapter| adapter.address);
    Ok(adapters)
}

fn read_adapter_devices(adapter_dir: &Path) -> Result<Vec<BluetoothDevice>> {
    let entries = fs::read_dir(adapter_dir).map_err(|source| BluebondError::Io {
        context: "reading BlueZ adapter directory",
        source,
    })?;

    let mut devices = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|source| BluebondError::Io {
            context: "reading BlueZ adapter entry",
            source,
        })?;
        let file_type = entry.file_type().map_err(|source| BluebondError::Io {
            context: "reading BlueZ adapter entry type",
            source,
        })?;

        if !file_type.is_dir() {
            continue;
        }

        let Some(device_address) = parse_dir_address(&entry.file_name()) else {
            debug!(
                path = %entry.path().display(),
                "skipping non-address BlueZ adapter entry"
            );
            continue;
        };

        let info_path = entry.path().join("info");
        if !info_path.is_file() {
            debug!(
                path = %entry.path().display(),
                "skipping BlueZ device directory without info file"
            );
            continue;
        }

        let info = fs::read_to_string(&info_path).map_err(|source| BluebondError::Io {
            context: "reading BlueZ device info file",
            source,
        })?;
        let sections = parse_info_sections(&info);
        devices.push(device_from_sections(device_address, &sections));
    }

    devices.sort_by_key(|device| device.address);
    Ok(devices)
}

fn parse_dir_address(name: &std::ffi::OsStr) -> Option<BluetoothAddress> {
    name.to_str()?.parse().ok()
}

fn device_from_sections(
    address: BluetoothAddress,
    sections: &BTreeMap<String, BTreeMap<String, String>>,
) -> BluetoothDevice {
    let general = sections.get("General");

    BluetoothDevice {
        address,
        name: general.and_then(|section| section.get("Name").cloned()),
        alias: general.and_then(|section| section.get("Alias").cloned()),
        address_type: general.and_then(|section| section.get("AddressType").cloned()),
        paired: general
            .and_then(|section| section.get("Paired").and_then(|value| parse_bool(value))),
        trusted: general
            .and_then(|section| section.get("Trusted").and_then(|value| parse_bool(value))),
        has_link_key: sections
            .get("LinkKey")
            .and_then(|section| section.get("Key"))
            .is_some_and(|key| !key.trim().is_empty()),
        has_long_term_key: sections
            .get("LongTermKey")
            .and_then(|section| section.get("Key"))
            .is_some_and(|key| !key.trim().is_empty()),
    }
}

fn parse_info_sections(input: &str) -> BTreeMap<String, BTreeMap<String, String>> {
    let mut sections = BTreeMap::<String, BTreeMap<String, String>>::new();
    let mut current_section: Option<String> = None;

    for line in input.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
            continue;
        }

        if let Some(section) = trimmed
            .strip_prefix('[')
            .and_then(|value| value.strip_suffix(']'))
        {
            let section = section.trim().to_string();
            sections.entry(section.clone()).or_default();
            current_section = Some(section);
            continue;
        }

        let Some(section) = current_section.as_ref() else {
            continue;
        };

        let Some((key, value)) = trimmed.split_once('=') else {
            continue;
        };

        sections
            .entry(section.clone())
            .or_default()
            .insert(key.trim().to_string(), value.trim().to_string());
    }

    sections
}

fn parse_bool(value: &str) -> Option<bool> {
    match value.trim() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}
