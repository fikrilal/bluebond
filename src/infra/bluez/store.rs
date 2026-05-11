use std::collections::BTreeMap;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use crate::domain::{BluetoothAdapter, BluetoothAddress, BluetoothDevice};
use crate::error::{BluebondError, Result};

pub const DEFAULT_BLUEZ_DIR: &str = "/var/lib/bluetooth";

pub fn default_store_exists() -> bool {
    Path::new(DEFAULT_BLUEZ_DIR).exists()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreReadiness {
    Readable,
    Missing,
    NotDirectory,
    PermissionDenied,
    Unreadable,
}

impl StoreReadiness {
    pub fn is_readable(self) -> bool {
        self == Self::Readable
    }
}

pub fn default_store_readiness() -> StoreReadiness {
    store_readiness(Path::new(DEFAULT_BLUEZ_DIR))
}

pub fn store_readiness(bluez_dir: &Path) -> StoreReadiness {
    if !bluez_dir.exists() {
        return StoreReadiness::Missing;
    }

    if !bluez_dir.is_dir() {
        return StoreReadiness::NotDirectory;
    }

    match fs::read_dir(bluez_dir) {
        Ok(_) => StoreReadiness::Readable,
        Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {
            StoreReadiness::PermissionDenied
        }
        Err(_) => StoreReadiness::Unreadable,
    }
}

pub fn read_inventory(bluez_dir: &Path) -> Result<Vec<BluetoothAdapter>> {
    let entries = fs::read_dir(bluez_dir).map_err(|source| {
        if source.kind() == std::io::ErrorKind::PermissionDenied {
            BluebondError::BluezStoreNotReadable {
                path: bluez_dir.to_path_buf(),
            }
        } else {
            BluebondError::Io {
                context: "reading BlueZ store directory",
                source,
            }
        }
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

pub fn read_device_info_content(
    bluez_dir: &Path,
    adapter_address: BluetoothAddress,
    device_address: BluetoothAddress,
) -> Result<Option<String>> {
    let info_path = bluez_dir
        .join(adapter_address.to_string())
        .join(device_address.to_string())
        .join("info");

    match fs::read_to_string(&info_path) {
        Ok(content) => Ok(Some(content)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(source) => Err(BluebondError::Io {
            context: "reading BlueZ device info file",
            source,
        }),
    }
}

pub fn write_device_info_atomic(info_path: &Path, content: &str) -> Result<()> {
    let Some(parent) = info_path.parent() else {
        return Err(BluebondError::Io {
            context: "resolving BlueZ info parent directory",
            source: std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "BlueZ info path has no parent directory",
            ),
        });
    };

    fs::create_dir_all(parent).map_err(|source| BluebondError::Io {
        context: "creating BlueZ device directory",
        source,
    })?;

    let temp_path = parent.join("info.bluebond.tmp");
    fs::write(&temp_path, content).map_err(|source| BluebondError::Io {
        context: "writing temporary BlueZ info file",
        source,
    })?;

    #[cfg(unix)]
    fs::set_permissions(&temp_path, fs::Permissions::from_mode(0o600)).map_err(|source| {
        BluebondError::Io {
            context: "setting BlueZ info permissions",
            source,
        }
    })?;

    fs::rename(&temp_path, info_path).map_err(|source| BluebondError::Io {
        context: "renaming BlueZ info file",
        source,
    })
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
            continue;
        };

        let info_path = entry.path().join("info");
        if !info_path.is_file() {
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
