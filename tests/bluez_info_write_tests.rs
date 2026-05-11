use std::path::PathBuf;

use bluebond::app::apply::{self, BluezInfoContentChange, BluezInfoContentPreview};
use bluebond::infra::bluez::store;

#[test]
fn writes_bluez_info_content_atomically_to_existing_record() {
    let temp = tempfile::tempdir().unwrap();
    let info_path = temp
        .path()
        .join("F8:89:D2:83:92:C0")
        .join("C6:C0:FD:F1:FB:80")
        .join("info");
    std::fs::create_dir_all(info_path.parent().unwrap()).unwrap();
    std::fs::write(&info_path, "[General]\nName=Old\n").unwrap();
    std::fs::write(info_path.parent().unwrap().join("attributes"), "keep").unwrap();
    let preview = preview_for_path(&info_path);

    let written = apply::write_bluez_info_records(&preview).unwrap();

    assert_eq!(written.files_written, vec![info_path.clone()]);
    assert_eq!(
        std::fs::read_to_string(&info_path).unwrap(),
        "[General]\nName=New\n"
    );
    assert_eq!(
        std::fs::read_to_string(info_path.parent().unwrap().join("attributes")).unwrap(),
        "keep"
    );
    assert!(!info_path
        .parent()
        .unwrap()
        .join("info.bluebond.tmp")
        .exists());
}

#[test]
fn creates_missing_bluez_device_directories_for_new_record() {
    let temp = tempfile::tempdir().unwrap();
    let info_path = temp
        .path()
        .join("F8:89:D2:83:92:C0")
        .join("C6:C0:FD:F1:FB:80")
        .join("info");
    let preview = preview_for_path(&info_path);

    apply::write_bluez_info_records(&preview).unwrap();

    assert_eq!(
        std::fs::read_to_string(&info_path).unwrap(),
        "[General]\nName=New\n"
    );
}

#[test]
fn skips_unchanged_bluez_info_records() {
    let temp = tempfile::tempdir().unwrap();
    let info_path = temp.path().join("F8:89:D2:83:92:C0/C6:C0:FD:F1:FB:80/info");
    let mut preview = preview_for_path(&info_path);
    preview.changes[0].content_changed = false;

    let written = apply::write_bluez_info_records(&preview).unwrap();

    assert!(written.files_written.is_empty());
    assert!(!info_path.exists());
}

#[test]
fn infra_writer_sets_conservative_permissions_on_unix() {
    let temp = tempfile::tempdir().unwrap();
    let info_path = temp.path().join("adapter/device/info");

    store::write_device_info_atomic(&info_path, "content").unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let mode = std::fs::metadata(&info_path).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
    }
}

fn preview_for_path(info_path: &std::path::Path) -> BluezInfoContentPreview {
    BluezInfoContentPreview {
        changes: vec![BluezInfoContentChange {
            display_name: "Legion M600 Mouse".to_string(),
            linux_adapter_address: "F8:89:D2:83:92:C0".parse().unwrap(),
            linux_target_device_address: "C6:C0:FD:F1:FB:80".parse().unwrap(),
            windows_source_device_address: "C6:C0:FD:F1:FB:80".parse().unwrap(),
            windows_source_registry_path: Some(
                r"ControlSet001\Services\BTHPORT\Parameters\Keys\f889d28392c0\c6c0fdf1fb80"
                    .to_string(),
            ),
            target_info_path: PathBuf::from(info_path),
            existing_info_content: Some("[General]\nName=Old\n".to_string()),
            next_info_content: "[General]\nName=New\n".to_string(),
            content_changed: true,
        }],
    }
}
