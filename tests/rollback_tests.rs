use std::path::{Path, PathBuf};

use bluebond::app::apply::{ApplySafetyMetadata, ApplySafetyMetadataChange};
use bluebond::app::rollback::{self, RollbackRestoreRequest};
use bluebond::infra::backup::store as backup_store;

#[test]
fn lists_bluebond_backups_from_metadata_files() {
    let temp = tempfile::tempdir().unwrap();
    let metadata = metadata_for_root(temp.path());
    write_metadata(temp.path(), "snapshot-1", &metadata);
    std::fs::create_dir_all(temp.path().join("not-a-backup")).unwrap();

    let list = rollback::list_backups(temp.path()).unwrap();

    assert_eq!(list.backups.len(), 1);
    assert_eq!(list.backups[0].snapshot_id, "snapshot-1");
    assert_eq!(list.backups[0].operation, "apply");
    assert_eq!(list.backups[0].changes.len(), 1);
    assert_eq!(list.backups[0].changes[0].display_name, "Mouse");
}

#[test]
fn restores_only_metadata_declared_backup_files() {
    let temp = tempfile::tempdir().unwrap();
    let metadata = metadata_for_root(temp.path());
    let backup_file = temp
        .path()
        .join("snapshot-1/F8:89:D2:83:92:C0/C6:C0:FD:F1:FB:80/info");
    std::fs::create_dir_all(backup_file.parent().unwrap()).unwrap();
    std::fs::write(
        &backup_file,
        "[General]\nName=Restored\n\n[LongTermKey]\nKey=ABC\n",
    )
    .unwrap();
    let target = temp
        .path()
        .join("bluez/F8:89:D2:83:92:C0/C6:C0:FD:F1:FB:80/info");
    std::fs::create_dir_all(target.parent().unwrap()).unwrap();
    std::fs::write(&target, "[General]\nName=Broken\n").unwrap();

    let restored = rollback::restore_metadata_files(&metadata).unwrap();

    assert_eq!(restored, vec![target.clone()]);
    assert!(std::fs::read_to_string(target)
        .unwrap()
        .contains("Name=Restored"));
}

#[test]
fn restore_fails_when_metadata_has_no_backup_path() {
    let temp = tempfile::tempdir().unwrap();
    let mut metadata = metadata_for_root(temp.path());
    metadata.changes[0].backup_path = None;

    let result = rollback::restore_metadata_files(&metadata);

    assert!(result.is_err());
}

#[test]
fn verifies_restored_state_from_metadata_targets() {
    let temp = tempfile::tempdir().unwrap();
    let metadata = metadata_for_root(temp.path());
    let target = temp
        .path()
        .join("bluez/F8:89:D2:83:92:C0/C6:C0:FD:F1:FB:80/info");
    std::fs::create_dir_all(target.parent().unwrap()).unwrap();
    std::fs::write(
        &target,
        "[General]\nName=Restored\n\n[LongTermKey]\nKey=ABC\n",
    )
    .unwrap();

    let report = rollback::verify_restored_state(&metadata).unwrap();

    assert!(report.all_targets_visible());
    assert_eq!(report.checked_devices.len(), 1);
    assert!(report
        .manual_reconnect_check
        .contains("reconnect the Bluetooth device"));
}

#[test]
fn reads_metadata_from_path() {
    let temp = tempfile::tempdir().unwrap();
    let metadata = metadata_for_root(temp.path());
    let metadata_path = write_metadata(temp.path(), "snapshot-1", &metadata);

    let parsed = rollback::read_metadata(&metadata_path).unwrap();

    assert_eq!(parsed.snapshot_id, "snapshot-1");
    assert_eq!(parsed.changes[0].display_name, "Mouse");
}

#[test]
fn rollback_restore_request_keeps_metadata_path() {
    let request = RollbackRestoreRequest {
        metadata_path: PathBuf::from("/tmp/bluebond-backup.json"),
    };

    assert!(request.metadata_path.ends_with("bluebond-backup.json"));
}

#[test]
fn privilege_gate_rejects_unprivileged_rollback() {
    assert!(rollback::require_privileged_rollback_with(false).is_err());
    assert!(rollback::require_privileged_rollback_with(true).is_ok());
}

fn write_metadata(root: &Path, snapshot_id: &str, metadata: &ApplySafetyMetadata) -> PathBuf {
    let snapshot_root = root.join(snapshot_id);
    let json = serde_json::to_string_pretty(metadata).unwrap();
    backup_store::write_metadata(&snapshot_root, &json).unwrap()
}

fn metadata_for_root(root: &Path) -> ApplySafetyMetadata {
    ApplySafetyMetadata {
        schema_version: 1,
        bluebond_version: "0.1.0".to_string(),
        operation: "apply".to_string(),
        snapshot_id: "snapshot-1".to_string(),
        backup_root: root.join("snapshot-1"),
        changes: vec![ApplySafetyMetadataChange {
            display_name: "Mouse".to_string(),
            linux_adapter_address: "F8:89:D2:83:92:C0".to_string(),
            linux_target_device_address: "C6:C0:FD:F1:FB:80".to_string(),
            windows_source_device_address: "C6:C0:FD:F1:FB:80".to_string(),
            windows_source_registry_path: Some(
                r"ControlSet001\Services\BTHPORT\Parameters\Keys\f889d28392c0\c6c0fdf1fb80"
                    .to_string(),
            ),
            target_info_path: root.join("bluez/F8:89:D2:83:92:C0/C6:C0:FD:F1:FB:80/info"),
            backup_path: Some(root.join("snapshot-1/F8:89:D2:83:92:C0/C6:C0:FD:F1:FB:80/info")),
            content_changed: true,
        }],
    }
}
