use std::path::PathBuf;

use bluebond::app::scan::{
    ScanReport, WindowsBluetoothAdapterKey, WindowsBluetoothDeviceKey,
    WindowsBluetoothKeyInspection, WindowsBluetoothKeyInspectionStatus,
};
use bluebond::domain::{BluetoothAdapter, BluetoothAddress, BluetoothDevice, KeyMaterialPresence};

#[test]
fn maps_scan_report_into_discovered_bond_state() {
    let linux_adapter_address: BluetoothAddress = "F8:89:D2:83:92:C0".parse().unwrap();
    let linux_device_address: BluetoothAddress = "C6:C0:FD:F1:FB:80".parse().unwrap();
    let windows_device_address: BluetoothAddress = "C6:C0:FA:F1:FB:80".parse().unwrap();

    let report = ScanReport {
        bluez_dir: PathBuf::from("tests/fixtures/bluez"),
        adapters: vec![BluetoothAdapter {
            address: linux_adapter_address,
            devices: vec![BluetoothDevice {
                address: linux_device_address,
                name: Some("Legion M600 Mouse".to_string()),
                alias: Some("M600".to_string()),
                address_type: Some("static".to_string()),
                paired: Some(true),
                trusted: Some(true),
                has_link_key: false,
                has_long_term_key: true,
            }],
        }],
        windows_candidates: Vec::new(),
        windows_bluetooth_keys: vec![WindowsBluetoothKeyInspection {
            hive_path: PathBuf::from("/mnt/windows/Windows/System32/config/SYSTEM"),
            status: WindowsBluetoothKeyInspectionStatus::Ready,
            adapters: vec![WindowsBluetoothAdapterKey {
                control_set: "ControlSet001".to_string(),
                registry_path: r"ControlSet001\Services\BTHPORT\Parameters\Keys\f889d28392c0"
                    .to_string(),
                adapter_address: linux_adapter_address,
                devices: vec![WindowsBluetoothDeviceKey {
                    registry_path:
                        r"ControlSet001\Services\BTHPORT\Parameters\Keys\f889d28392c0\c6c0faf1fb80"
                            .to_string(),
                    device_address: windows_device_address,
                    has_key_material: true,
                }],
            }],
        }],
    };

    let state = report.discovered_bond_state();

    assert_eq!(state.linux_adapters.len(), 1);
    assert_eq!(state.linux_adapters[0].address, linux_adapter_address);
    assert_eq!(
        state.linux_adapters[0].devices[0].address,
        linux_device_address
    );
    assert_eq!(state.linux_adapters[0].devices[0].display_name, "M600");
    assert_eq!(
        state.linux_adapters[0].devices[0].key_material.link_key,
        KeyMaterialPresence::NotPresent
    );
    assert_eq!(
        state.linux_adapters[0].devices[0]
            .key_material
            .long_term_key,
        KeyMaterialPresence::Present
    );

    assert_eq!(state.windows_adapters.len(), 1);
    assert_eq!(state.windows_adapters[0].address, linux_adapter_address);
    assert_eq!(state.windows_adapters[0].control_set, "ControlSet001");
    assert_eq!(
        state.windows_adapters[0].devices[0].address,
        windows_device_address
    );
    assert_eq!(
        state.windows_adapters[0].devices[0].key_material,
        KeyMaterialPresence::Present
    );
}
