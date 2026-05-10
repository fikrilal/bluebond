use bluebond::domain::{
    AdapterMatchStatus, BluetoothAddress, BondMatchReport, DeviceMatchStatus, DiscoveredBondState,
    KeyMaterialPresence, LinuxBondAdapter, LinuxBondDevice, LinuxKeyMaterial, WindowsBondAdapter,
    WindowsBondDevice,
};

#[test]
fn matches_exact_adapter_and_device_addresses() {
    let state = state_with(
        linux_adapter("F8:89:D2:83:92:C0", vec![linux_device("C6:C0:FD:F1:FB:80")]),
        vec![windows_adapter(
            "F8:89:D2:83:92:C0",
            vec![windows_device(
                "C6:C0:FD:F1:FB:80",
                KeyMaterialPresence::Present,
            )],
        )],
    );

    let report = BondMatchReport::exact_from(&state);

    assert_eq!(report.adapters.len(), 1);
    assert_eq!(report.adapters[0].status, AdapterMatchStatus::Exact);
    assert_eq!(report.adapters[0].devices.len(), 1);
    assert_eq!(
        report.adapters[0].devices[0].status,
        DeviceMatchStatus::ExactUsable
    );
    assert_eq!(
        report.adapters[0].devices[0].windows_address,
        Some(address("C6:C0:FD:F1:FB:80"))
    );
}

#[test]
fn reports_missing_windows_adapter() {
    let state = state_with(
        linux_adapter("F8:89:D2:83:92:C0", vec![linux_device("C6:C0:FD:F1:FB:80")]),
        Vec::new(),
    );

    let report = BondMatchReport::exact_from(&state);

    assert_eq!(
        report.adapters[0].status,
        AdapterMatchStatus::MissingWindowsAdapter
    );
    assert_eq!(
        report.adapters[0].devices[0].status,
        DeviceMatchStatus::MissingWindowsAdapter
    );
}

#[test]
fn reports_missing_windows_device_inside_matched_adapter() {
    let state = state_with(
        linux_adapter("F8:89:D2:83:92:C0", vec![linux_device("C6:C0:FD:F1:FB:80")]),
        vec![windows_adapter("F8:89:D2:83:92:C0", Vec::new())],
    );

    let report = BondMatchReport::exact_from(&state);

    assert_eq!(
        report.adapters[0].devices[0].status,
        DeviceMatchStatus::MissingWindowsDevice
    );
}

#[test]
fn reports_exact_device_match_without_windows_key_material_as_not_usable() {
    let state = state_with(
        linux_adapter("F8:89:D2:83:92:C0", vec![linux_device("C6:C0:FD:F1:FB:80")]),
        vec![windows_adapter(
            "F8:89:D2:83:92:C0",
            vec![windows_device(
                "C6:C0:FD:F1:FB:80",
                KeyMaterialPresence::NotPresent,
            )],
        )],
    );

    let report = BondMatchReport::exact_from(&state);

    assert_eq!(
        report.adapters[0].devices[0].status,
        DeviceMatchStatus::ExactMissingWindowsKeyMaterial
    );
}

fn state_with(
    linux_adapter: LinuxBondAdapter,
    windows_adapters: Vec<WindowsBondAdapter>,
) -> DiscoveredBondState {
    DiscoveredBondState::new(vec![linux_adapter], windows_adapters)
}

fn linux_adapter(address: &str, devices: Vec<LinuxBondDevice>) -> LinuxBondAdapter {
    LinuxBondAdapter::new(address.parse().unwrap(), devices)
}

fn linux_device(address: &str) -> LinuxBondDevice {
    LinuxBondDevice {
        address: address.parse().unwrap(),
        display_name: "Legion M600 Mouse".to_string(),
        address_type: Some("static".to_string()),
        paired: Some(true),
        trusted: Some(true),
        key_material: LinuxKeyMaterial {
            link_key: KeyMaterialPresence::NotPresent,
            long_term_key: KeyMaterialPresence::Present,
        },
    }
}

fn windows_adapter(address: &str, devices: Vec<WindowsBondDevice>) -> WindowsBondAdapter {
    WindowsBondAdapter::new(
        address.parse().unwrap(),
        "ControlSet001",
        format!(
            r"ControlSet001\Services\BTHPORT\Parameters\Keys\{}",
            address.replace(':', "").to_ascii_lowercase()
        ),
        devices,
    )
}

fn windows_device(address: &str, key_material: KeyMaterialPresence) -> WindowsBondDevice {
    WindowsBondDevice {
        address: address.parse().unwrap(),
        registry_path: format!(
            r"ControlSet001\Services\BTHPORT\Parameters\Keys\f889d28392c0\{}",
            address.replace(':', "").to_ascii_lowercase()
        ),
        key_material,
    }
}

fn address(value: &str) -> BluetoothAddress {
    value.parse().unwrap()
}
