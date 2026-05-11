use std::path::PathBuf;

use bluebond::app::apply::{
    self, BluezInfoPreviewRequest, ExistingBluezInfoContent, WindowsDeviceKeyMaterial,
};
use bluebond::app::scan::ScanReport;
use bluebond::convert::windows_key_material;
use bluebond::domain::{
    BluetoothAdapter, BluetoothAddress, SyncPlan, SyncPlanAction, SyncPlanActionType,
};
use bluebond::infra::bluez::store;

#[test]
fn previews_updated_bluez_info_content_for_existing_record() {
    let plan = plan_with_action(SyncPlanActionType::UpdateExistingBluezRecord);
    let request = base_request().with_existing_infos(vec![ExistingBluezInfoContent {
        linux_adapter_address: adapter_address(),
        linux_device_address: linux_device_address(),
        content: "[General]\nName=Legion M600 Mouse\n\n[LongTermKey]\nKey=OLD\n".to_string(),
    }]);

    let preview = apply::preview_bluez_info_content(&plan, &request).unwrap();

    assert_eq!(preview.changes.len(), 1);
    assert_eq!(
        preview.changes[0].target_info_path,
        PathBuf::from("tests/fixtures/bluez/F8:89:D2:83:92:C0/C6:C0:FD:F1:FB:80/info")
    );
    assert!(preview.changes[0].existing_info_content.is_some());
    assert!(preview.changes[0].content_changed);
    assert!(preview.changes[0].next_info_content.contains("[General]\n"));
    assert!(preview.changes[0]
        .next_info_content
        .contains("[IdentityResolvingKey]\n"));
    assert!(preview.changes[0]
        .next_info_content
        .contains("Key=00112233445566778899AABBCCDDEEFF\n"));
    assert!(!preview.changes[0].next_info_content.contains("Key=OLD"));
}

#[test]
fn previews_new_bluez_info_content_for_create_action() {
    let plan = plan_with_action(SyncPlanActionType::CreateBluezRecord);
    let request = base_request();

    let preview = apply::preview_bluez_info_content(&plan, &request).unwrap();

    assert_eq!(preview.changes[0].existing_info_content, None);
    assert!(preview.changes[0].content_changed);
    assert!(preview.changes[0]
        .next_info_content
        .starts_with("[IdentityResolvingKey]\n"));
}

#[test]
fn reports_unchanged_content_when_preview_matches_existing_info() {
    let plan = plan_with_action(SyncPlanActionType::UpdateExistingBluezRecord);
    let rendered_content = rendered_key_content();
    let request = base_request().with_existing_infos(vec![ExistingBluezInfoContent {
        linux_adapter_address: adapter_address(),
        linux_device_address: linux_device_address(),
        content: rendered_content,
    }]);

    let preview = apply::preview_bluez_info_content(&plan, &request).unwrap();

    assert!(!preview.changes[0].content_changed);
}

#[test]
fn errors_when_action_has_no_windows_key_material() {
    let plan = plan_with_action(SyncPlanActionType::UpdateExistingBluezRecord);
    let request = BluezInfoPreviewRequest::new("tests/fixtures/bluez");

    let result = apply::preview_bluez_info_content(&plan, &request);

    assert!(result.is_err());
}

#[test]
fn reads_existing_bluez_info_content_for_preview_collection() {
    let content = store::read_device_info_content(
        &PathBuf::from("tests/fixtures/bluez"),
        adapter_address(),
        linux_device_address(),
    )
    .unwrap();

    assert!(content.unwrap().contains("[General]\n"));
}

#[test]
fn preview_collection_reports_missing_windows_source_material() {
    let scan_report = ScanReport {
        bluez_dir: PathBuf::from("tests/fixtures/bluez"),
        adapters: vec![BluetoothAdapter {
            address: adapter_address(),
            devices: Vec::new(),
        }],
        windows_candidates: Vec::new(),
        windows_bluetooth_keys: Vec::new(),
    };
    let plan = plan_with_action(SyncPlanActionType::UpdateExistingBluezRecord);

    let result = apply::collect_preview_request(&scan_report, &plan);

    assert!(result.is_err());
}

#[test]
fn debug_redacts_preview_file_content() {
    let plan = plan_with_action(SyncPlanActionType::UpdateExistingBluezRecord);
    let request = base_request().with_existing_infos(vec![ExistingBluezInfoContent {
        linux_adapter_address: adapter_address(),
        linux_device_address: linux_device_address(),
        content: "[General]\nName=Legion M600 Mouse\n".to_string(),
    }]);

    let preview = apply::preview_bluez_info_content(&plan, &request).unwrap();
    let debug = format!("{preview:?}");

    assert!(debug.contains("<redacted>"));
    assert!(!debug.contains("00112233445566778899AABBCCDDEEFF"));
    assert!(!debug.contains("Legion M600 Mouse\\n"));
}

fn base_request() -> BluezInfoPreviewRequest {
    BluezInfoPreviewRequest::new("tests/fixtures/bluez").with_windows_key_materials(vec![
        WindowsDeviceKeyMaterial {
            linux_adapter_address: adapter_address(),
            windows_device_address: windows_device_address(),
            registry_path: Some(
                r"ControlSet001\Services\BTHPORT\Parameters\Keys\f889d28392c0\c6c0fdf1fb80"
                    .to_string(),
            ),
            material: windows_key_material(),
        },
    ])
}

fn rendered_key_content() -> String {
    apply::preview_bluez_info_content(
        &plan_with_action(SyncPlanActionType::CreateBluezRecord),
        &base_request(),
    )
    .unwrap()
    .changes
    .remove(0)
    .next_info_content
}

fn plan_with_action(action_type: SyncPlanActionType) -> SyncPlan {
    SyncPlan {
        actions: vec![SyncPlanAction {
            action_type,
            linux_adapter_address: adapter_address(),
            linux_target_device_address: linux_device_address(),
            windows_source_device_address: windows_device_address(),
            display_name: "Legion M600 Mouse".to_string(),
        }],
        skipped: Vec::new(),
    }
}

fn windows_key_material() -> bluebond::convert::windows_key_material::WindowsBluetoothKeyMaterial {
    windows_key_material::parse_hivexsh_lsval_output(
        r#"
"LTK"=hex(3):00,11,22,33,44,55,66,77,88,99,aa,bb,cc,dd,ee,ff
"KeyLength"=dword:00000010
"ERand"=hex(11):10,20,30,40,50,60,70,80
"EDIV"=dword:000071b9
"IRK"=hex(3):ff,ee,dd,cc,bb,aa,99,88,77,66,55,44,33,22,11,00
"CSRK"=hex(3):12,34,56,78,90,ab,cd,ef,fe,dc,ba,09,87,65,43,21
"AuthReq"=dword:0000002d
"#,
    )
    .unwrap()
}

fn adapter_address() -> BluetoothAddress {
    "F8:89:D2:83:92:C0".parse().unwrap()
}

fn linux_device_address() -> BluetoothAddress {
    "C6:C0:FD:F1:FB:80".parse().unwrap()
}

fn windows_device_address() -> BluetoothAddress {
    "C6:C0:FD:F1:FB:80".parse().unwrap()
}
