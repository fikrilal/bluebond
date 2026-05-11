use std::path::PathBuf;

use bluebond::app::apply::{self, BluezInfoContentChange, BluezInfoContentPreview};

#[test]
fn verifies_post_apply_bluez_record_visibility() {
    let temp = tempfile::tempdir().unwrap();
    let info_path = write_info(
        temp.path(),
        "[General]\nName=Mouse\n\n[LongTermKey]\nKey=ABC\n",
    );
    let preview = preview_for_path(&info_path);

    let report = apply::verify_post_apply_state(temp.path(), &preview).unwrap();

    assert!(report.all_expected_records_visible());
    assert_eq!(report.checked_devices.len(), 1);
    assert!(report.checked_devices[0].found);
    assert!(report.checked_devices[0].has_long_term_key);
    assert!(report
        .manual_reconnect_check
        .contains("reconnect the Bluetooth device"));
}

#[test]
fn reports_missing_post_apply_target_record() {
    let temp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(temp.path()).unwrap();
    let preview = preview_for_path(&temp.path().join("F8:89:D2:83:92:C0/C6:C0:FD:F1:FB:80/info"));

    let report = apply::verify_post_apply_state(temp.path(), &preview).unwrap();

    assert!(!report.all_expected_records_visible());
    assert!(!report.checked_devices[0].found);
}

fn write_info(root: &std::path::Path, content: &str) -> PathBuf {
    let info_path = root.join("F8:89:D2:83:92:C0/C6:C0:FD:F1:FB:80/info");
    std::fs::create_dir_all(info_path.parent().unwrap()).unwrap();
    std::fs::write(&info_path, content).unwrap();
    info_path
}

fn preview_for_path(info_path: &std::path::Path) -> BluezInfoContentPreview {
    BluezInfoContentPreview {
        changes: vec![BluezInfoContentChange {
            display_name: "Mouse".to_string(),
            linux_adapter_address: "F8:89:D2:83:92:C0".parse().unwrap(),
            linux_target_device_address: "C6:C0:FD:F1:FB:80".parse().unwrap(),
            windows_source_device_address: "C6:C0:FD:F1:FB:80".parse().unwrap(),
            windows_source_registry_path: Some(
                r"ControlSet001\Services\BTHPORT\Parameters\Keys\f889d28392c0\c6c0fdf1fb80"
                    .to_string(),
            ),
            target_info_path: PathBuf::from(info_path),
            existing_info_content: None,
            next_info_content: "[LongTermKey]\nKey=ABC\n".to_string(),
            content_changed: true,
        }],
    }
}
