use bluebond::domain::{
    BondMatchReport, DiscoveredBondState, KeyMaterialPresence, LinuxBondAdapter, LinuxBondDevice,
    LinuxKeyMaterial, SkipReason, SyncPlan, SyncPlanActionType, WindowsBondAdapter,
    WindowsBondDevice,
};

#[test]
fn exact_usable_match_generates_update_action() {
    let report = match_report(
        "C6:C0:FD:F1:FB:80",
        vec![windows_device(
            "C6:C0:FD:F1:FB:80",
            KeyMaterialPresence::Present,
        )],
    );

    let plan = SyncPlan::from_match_report(&report);

    assert_eq!(plan.actions.len(), 1);
    assert_eq!(
        plan.actions[0].action_type,
        SyncPlanActionType::UpdateExistingBluezRecord
    );
    assert_eq!(
        plan.actions[0].linux_target_device_address.to_string(),
        "C6:C0:FD:F1:FB:80"
    );
    assert_eq!(
        plan.actions[0].windows_source_device_address.to_string(),
        "C6:C0:FD:F1:FB:80"
    );
    assert!(plan.skipped.is_empty());
}

#[test]
fn address_drift_candidate_generates_create_action() {
    let report = match_report(
        "C6:C0:FC:F1:FB:80",
        vec![windows_device(
            "C6:C0:FD:F1:FB:80",
            KeyMaterialPresence::Present,
        )],
    );

    let plan = SyncPlan::from_match_report(&report);

    assert_eq!(plan.actions.len(), 1);
    assert_eq!(
        plan.actions[0].action_type,
        SyncPlanActionType::CreateBluezRecord
    );
    assert_eq!(
        plan.actions[0].linux_target_device_address.to_string(),
        "C6:C0:FC:F1:FB:80"
    );
    assert_eq!(
        plan.actions[0].windows_source_device_address.to_string(),
        "C6:C0:FD:F1:FB:80"
    );
}

#[test]
fn ambiguous_drift_is_skipped() {
    let report = match_report(
        "C6:C0:FC:F1:FB:80",
        vec![
            windows_device("C6:C0:FD:F1:FB:80", KeyMaterialPresence::Present),
            windows_device("C6:C0:FE:F1:FB:80", KeyMaterialPresence::Present),
        ],
    );

    let plan = SyncPlan::from_match_report(&report);

    assert!(plan.actions.is_empty());
    assert_eq!(plan.skipped.len(), 1);
    assert_eq!(plan.skipped[0].reason, SkipReason::AmbiguousAddressDrift);
}

#[test]
fn missing_windows_key_material_is_skipped() {
    let report = match_report(
        "C6:C0:FD:F1:FB:80",
        vec![windows_device(
            "C6:C0:FD:F1:FB:80",
            KeyMaterialPresence::NotPresent,
        )],
    );

    let plan = SyncPlan::from_match_report(&report);

    assert!(plan.actions.is_empty());
    assert_eq!(
        plan.skipped[0].reason,
        SkipReason::MissingWindowsKeyMaterial
    );
}

#[test]
fn missing_windows_device_is_skipped() {
    let report = match_report("C6:C0:FD:F1:FB:80", Vec::new());

    let plan = SyncPlan::from_match_report(&report);

    assert!(plan.actions.is_empty());
    assert_eq!(plan.skipped[0].reason, SkipReason::MissingWindowsDevice);
}

#[test]
fn missing_windows_adapter_is_skipped() {
    let state = DiscoveredBondState::new(
        vec![linux_adapter("F8:89:D2:83:92:C0", "C6:C0:FD:F1:FB:80")],
        Vec::new(),
    );
    let report = BondMatchReport::exact_from(&state);

    let plan = SyncPlan::from_match_report(&report);

    assert!(plan.actions.is_empty());
    assert_eq!(plan.skipped[0].reason, SkipReason::MissingWindowsAdapter);
}

fn match_report(
    linux_device_address: &str,
    windows_devices: Vec<WindowsBondDevice>,
) -> BondMatchReport {
    let state = DiscoveredBondState::new(
        vec![linux_adapter("F8:89:D2:83:92:C0", linux_device_address)],
        vec![windows_adapter("F8:89:D2:83:92:C0", windows_devices)],
    );

    BondMatchReport::exact_from(&state)
}

fn linux_adapter(adapter_address: &str, device_address: &str) -> LinuxBondAdapter {
    LinuxBondAdapter::new(
        adapter_address.parse().unwrap(),
        vec![LinuxBondDevice {
            address: device_address.parse().unwrap(),
            display_name: "Legion M600 Mouse".to_string(),
            address_type: Some("static".to_string()),
            paired: Some(true),
            trusted: Some(true),
            key_material: LinuxKeyMaterial {
                link_key: KeyMaterialPresence::NotPresent,
                long_term_key: KeyMaterialPresence::Present,
            },
        }],
    )
}

fn windows_adapter(adapter_address: &str, devices: Vec<WindowsBondDevice>) -> WindowsBondAdapter {
    WindowsBondAdapter::new(
        adapter_address.parse().unwrap(),
        "ControlSet001",
        r"ControlSet001\Services\BTHPORT\Parameters\Keys\f889d28392c0",
        devices,
    )
}

fn windows_device(device_address: &str, key_material: KeyMaterialPresence) -> WindowsBondDevice {
    WindowsBondDevice {
        address: device_address.parse().unwrap(),
        registry_path: format!(
            r"ControlSet001\Services\BTHPORT\Parameters\Keys\f889d28392c0\{}",
            device_address.replace(':', "").to_ascii_lowercase()
        ),
        key_material,
    }
}
