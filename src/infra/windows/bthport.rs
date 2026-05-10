use std::path::{Path, PathBuf};

use crate::domain::BluetoothAddress;
use crate::infra::command;

const CONTROL_SET_RANGE: std::ops::RangeInclusive<u8> = 1..=9;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WindowsBluetoothKeyInspection {
    pub hive_path: PathBuf,
    pub status: WindowsBluetoothKeyInspectionStatus,
    pub adapters: Vec<WindowsBluetoothAdapterKey>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum WindowsBluetoothKeyInspectionStatus {
    Ready,
    MissingTool,
    NoKeysFound,
    CommandFailed,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WindowsBluetoothAdapterKey {
    pub control_set: String,
    pub registry_path: String,
    pub adapter_address: BluetoothAddress,
    pub devices: Vec<WindowsBluetoothDeviceKey>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WindowsBluetoothDeviceKey {
    pub registry_path: String,
    pub device_address: BluetoothAddress,
    pub has_key_material: bool,
}

pub fn inspect_adapter_keys(hive_path: &Path) -> WindowsBluetoothKeyInspection {
    if !command::exists("hivexsh") {
        return WindowsBluetoothKeyInspection {
            hive_path: hive_path.to_path_buf(),
            status: WindowsBluetoothKeyInspectionStatus::MissingTool,
            adapters: Vec::new(),
        };
    }

    let mut adapters = Vec::new();
    let mut saw_command_failure = false;
    let hive = hive_path.to_string_lossy();

    for control_set_number in CONTROL_SET_RANGE {
        let control_set = format!("ControlSet{control_set_number:03}");
        let registry_path = format!(r"{control_set}\Services\BTHPORT\Parameters\Keys");
        let script = format!("cd \\{registry_path}\nls\nquit\n");

        match command::run_with_stdin("hivexsh", &[hive.as_ref()], &script) {
            Ok(output) => {
                let mut parsed_adapters =
                    parse_adapter_key_listing(&control_set, &registry_path, &output.stdout);

                for adapter in &mut parsed_adapters {
                    adapter.devices =
                        inspect_device_keys_for_adapter(hive_path, &adapter.registry_path);
                }

                adapters.extend(parsed_adapters);
            }
            Err(_) => {
                saw_command_failure = true;
            }
        }
    }

    let status = if !adapters.is_empty() {
        WindowsBluetoothKeyInspectionStatus::Ready
    } else if saw_command_failure {
        WindowsBluetoothKeyInspectionStatus::CommandFailed
    } else {
        WindowsBluetoothKeyInspectionStatus::NoKeysFound
    };

    WindowsBluetoothKeyInspection {
        hive_path: hive_path.to_path_buf(),
        status,
        adapters,
    }
}

pub fn parse_adapter_key_listing(
    control_set: &str,
    registry_path: &str,
    output: &str,
) -> Vec<WindowsBluetoothAdapterKey> {
    let mut adapters = output
        .lines()
        .filter_map(|line| parse_adapter_key_line(control_set, registry_path, line))
        .collect::<Vec<_>>();

    adapters.sort_by_key(|adapter| adapter.adapter_address);
    adapters.dedup_by_key(|adapter| adapter.adapter_address);
    adapters
}

fn parse_adapter_key_line(
    control_set: &str,
    registry_path: &str,
    line: &str,
) -> Option<WindowsBluetoothAdapterKey> {
    let key_name = line.trim();
    let adapter_address = parse_compact_registry_address(key_name)?;

    Some(WindowsBluetoothAdapterKey {
        control_set: control_set.to_string(),
        registry_path: format!(r"{registry_path}\{}", key_name.to_ascii_lowercase()),
        adapter_address,
        devices: Vec::new(),
    })
}

fn inspect_device_keys_for_adapter(
    hive_path: &Path,
    adapter_registry_path: &str,
) -> Vec<WindowsBluetoothDeviceKey> {
    let script = format!("cd \\{adapter_registry_path}\nls\nquit\n");
    let hive = hive_path.to_string_lossy();

    let Ok(output) = command::run_with_stdin("hivexsh", &[hive.as_ref()], &script) else {
        return Vec::new();
    };

    let mut devices = parse_device_key_listing(adapter_registry_path, &output.stdout);

    for device in &mut devices {
        let script = format!("cd \\{}\nlsval\nquit\n", device.registry_path);
        let Ok(output) = command::run_with_stdin("hivexsh", &[hive.as_ref()], &script) else {
            continue;
        };

        device.has_key_material = parse_key_material_presence(&output.stdout);
    }

    devices
}

pub fn parse_device_key_listing(
    adapter_registry_path: &str,
    output: &str,
) -> Vec<WindowsBluetoothDeviceKey> {
    let mut devices = output
        .lines()
        .filter_map(|line| parse_device_key_line(adapter_registry_path, line))
        .collect::<Vec<_>>();

    devices.sort_by_key(|device| device.device_address);
    devices.dedup_by_key(|device| device.device_address);
    devices
}

pub fn parse_key_material_presence(output: &str) -> bool {
    output.lines().any(|line| {
        let trimmed = line.trim();

        trimmed.starts_with("\"LTK\"=")
            || trimmed.starts_with("\"Key\"=")
            || trimmed.starts_with("\"IRK\"=")
            || trimmed.starts_with("\"CSRK\"=")
    })
}

fn parse_device_key_line(
    adapter_registry_path: &str,
    line: &str,
) -> Option<WindowsBluetoothDeviceKey> {
    let key_name = line.trim();
    let device_address = parse_compact_registry_address(key_name)?;

    Some(WindowsBluetoothDeviceKey {
        registry_path: format!(r"{adapter_registry_path}\{}", key_name.to_ascii_lowercase()),
        device_address,
        has_key_material: false,
    })
}

fn parse_compact_registry_address(key_name: &str) -> Option<BluetoothAddress> {
    if key_name.len() != 12 || !key_name.chars().all(|value| value.is_ascii_hexdigit()) {
        return None;
    }

    BluetoothAddress::from_compact_hex(key_name).ok()
}
