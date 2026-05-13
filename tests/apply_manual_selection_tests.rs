use std::path::PathBuf;

use bluebond::app::apply::{self, ManualApplySelection, ManualApplyTargetMode};
use bluebond::app::scan::{
    ScanReport, WindowsBluetoothAdapterKey, WindowsBluetoothDeviceKey,
    WindowsBluetoothKeyInspection, WindowsBluetoothKeyInspectionStatus,
};
use bluebond::domain::{BluetoothAdapter, BluetoothAddress, BluetoothDevice, SyncPlanActionType};

#[test]
fn manual_selection_builds_single_sync_action_for_ambiguous_target() {
    let report = scan_report();
    let selection =
        ManualApplySelection::from_raw(None, "C6:C0:FE:F1:FB:80", "C6:C0:FD:F1:FB:80").unwrap();

    let plan = apply::build_manual_sync_plan(&report, &selection).unwrap();

    assert_eq!(plan.actions.len(), 1);
    assert_eq!(
        plan.actions[0].action_type,
        SyncPlanActionType::UpdateExistingBluezRecord
    );
    assert_eq!(
        plan.actions[0].linux_target_device_address.to_string(),
        "C6:C0:FE:F1:FB:80"
    );
    assert_eq!(
        plan.actions[0].windows_source_device_address.to_string(),
        "C6:C0:FD:F1:FB:80"
    );
    assert_eq!(plan.actions[0].display_name, "Legion M600 Mouse");
    assert!(plan.skipped.is_empty());
}

#[test]
fn manual_selection_rejects_missing_linux_target() {
    let report = scan_report();
    let selection =
        ManualApplySelection::from_raw(None, "AA:AA:AA:AA:AA:AA", "C6:C0:FD:F1:FB:80").unwrap();

    let result = apply::build_manual_sync_plan(&report, &selection);

    assert!(result.is_err());
}

#[test]
fn manual_selection_rejects_missing_windows_source() {
    let report = scan_report();
    let selection =
        ManualApplySelection::from_raw(None, "C6:C0:FE:F1:FB:80", "AA:AA:AA:AA:AA:AA").unwrap();

    let result = apply::build_manual_sync_plan(&report, &selection);

    assert!(result.is_err());
}

#[test]
fn manual_selection_rejects_windows_source_without_key_material() {
    let mut report = scan_report();
    report.windows_bluetooth_keys[0].adapters[0].devices[2].has_key_material = false;
    let selection =
        ManualApplySelection::from_raw(None, "C6:C0:FE:F1:FB:80", "C6:C0:FD:F1:FB:80").unwrap();

    let result = apply::build_manual_sync_plan(&report, &selection);

    assert!(result.is_err());
}

#[test]
fn manual_selection_uses_adapter_to_disambiguate_duplicate_linux_targets() {
    let mut report = scan_report();
    report.adapters.push(BluetoothAdapter {
        address: address("AA:BB:CC:DD:EE:FF"),
        devices: vec![linux_device("C6:C0:FE:F1:FB:80", "Duplicate Mouse")],
    });
    let selection = ManualApplySelection::from_raw(
        Some("F8:89:D2:83:92:C0"),
        "C6:C0:FE:F1:FB:80",
        "C6:C0:FD:F1:FB:80",
    )
    .unwrap();

    let plan = apply::build_manual_sync_plan(&report, &selection).unwrap();

    assert_eq!(
        plan.actions[0].linux_adapter_address.to_string(),
        "F8:89:D2:83:92:C0"
    );
}

#[test]
fn manual_selection_can_target_windows_source_address_experimentally() {
    let report = scan_report();
    let selection = ManualApplySelection::from_raw_with_target_mode(
        None,
        "C6:C0:FE:F1:FB:80",
        "C6:C0:FD:F1:FB:80",
        ManualApplyTargetMode::WindowsSource,
    )
    .unwrap();

    let plan = apply::build_manual_sync_plan(&report, &selection).unwrap();

    assert_eq!(
        plan.actions[0].action_type,
        SyncPlanActionType::CreateBluezRecord
    );
    assert_eq!(
        plan.actions[0].linux_target_device_address.to_string(),
        "C6:C0:FD:F1:FB:80"
    );
    assert_eq!(
        plan.actions[0]
            .bluez_template_device_address
            .unwrap()
            .to_string(),
        "C6:C0:FE:F1:FB:80"
    );
    assert_eq!(plan.actions[0].display_name, "Legion M600 Mouse");
}

fn scan_report() -> ScanReport {
    ScanReport {
        bluez_dir: PathBuf::from("tests/fixtures/bluez"),
        adapters: vec![BluetoothAdapter {
            address: address("F8:89:D2:83:92:C0"),
            devices: vec![
                linux_device("C6:C0:FE:F1:FB:80", "Legion M600 Mouse"),
                linux_device("D7:54:3E:AD:9C:91", "REXUS KL150-BT2"),
            ],
        }],
        windows_candidates: Vec::new(),
        windows_bluetooth_keys: vec![WindowsBluetoothKeyInspection {
            hive_path: PathBuf::from("tests/fixtures/windows/Windows/System32/config/SYSTEM"),
            status: WindowsBluetoothKeyInspectionStatus::Ready,
            adapters: vec![WindowsBluetoothAdapterKey {
                control_set: "ControlSet001".to_string(),
                registry_path: r"ControlSet001\Services\BTHPORT\Parameters\Keys\f889d28392c0"
                    .to_string(),
                adapter_address: address("F8:89:D2:83:92:C0"),
                devices: vec![
                    windows_device("C6:C0:F8:F1:FB:80", true),
                    windows_device("C6:C0:FA:F1:FB:80", true),
                    windows_device("C6:C0:FD:F1:FB:80", true),
                ],
            }],
        }],
    }
}

fn linux_device(address: &str, name: &str) -> BluetoothDevice {
    BluetoothDevice {
        address: self::address(address),
        name: Some(name.to_string()),
        alias: None,
        address_type: Some("public".to_string()),
        paired: Some(true),
        trusted: Some(true),
        has_link_key: false,
        has_long_term_key: true,
    }
}

fn windows_device(address: &str, has_key_material: bool) -> WindowsBluetoothDeviceKey {
    WindowsBluetoothDeviceKey {
        registry_path: format!(
            r"ControlSet001\Services\BTHPORT\Parameters\Keys\f889d28392c0\{}",
            address.replace(':', "").to_ascii_lowercase()
        ),
        device_address: self::address(address),
        has_key_material,
    }
}

fn address(value: &str) -> BluetoothAddress {
    value.parse().unwrap()
}
