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
                adapters.extend(parse_adapter_key_listing(
                    &control_set,
                    &registry_path,
                    &output.stdout,
                ));
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

    if key_name.len() != 12 || !key_name.chars().all(|value| value.is_ascii_hexdigit()) {
        return None;
    }

    let adapter_address = BluetoothAddress::from_compact_hex(key_name).ok()?;

    Some(WindowsBluetoothAdapterKey {
        control_set: control_set.to_string(),
        registry_path: format!(r"{registry_path}\{}", key_name.to_ascii_lowercase()),
        adapter_address,
    })
}
